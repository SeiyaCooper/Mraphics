use std::collections::HashMap;

#[derive(Debug)]
pub struct Gadget {
    pub ty: wgpu::BufferBindingType,
    buffer: wgpu::Buffer,
}

#[derive(Debug, Clone, Copy)]
pub struct GadgetIndex {
    pub group_index: usize,
    pub binding_index: u32,
}

pub struct GadgetDescriptor<'a> {
    pub label: &'a str,
    pub index: GadgetIndex,
    pub size: u64,
    pub usage: wgpu::BufferUsages,
    pub ty: wgpu::BufferBindingType,
}

#[derive(Clone, Debug)]
pub struct GadgetData {
    pub label: String,
    pub index: GadgetIndex,
    pub data: Vec<u8>,
    pub needs_update_value: bool,
    pub needs_update_buffer: bool,
}

#[derive(Debug)]
pub struct Bundle {
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

#[derive(Debug)]
pub enum ConveyorError {
    UnknownGadgetLabel,
}

pub struct Conveyor {
    pub needs_update: bool,
    pub bundles: Vec<Option<Bundle>>,

    gadgets: HashMap<String, Gadget>,
    indices: Vec<Option<HashMap<u32, String>>>,
}

impl Conveyor {
    pub fn new() -> Self {
        Self {
            gadgets: HashMap::new(),
            bundles: Vec::new(),
            indices: Vec::new(),
            needs_update: false,
        }
    }

    /// Updates or inserts a gadget and marks self as requiring an update
    pub fn upsert_gadget(&mut self, device: &wgpu::Device, desc: &GadgetDescriptor) {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(desc.label),
            size: desc.size,
            usage: desc.usage,
            mapped_at_creation: false,
        });

        let gadget = Gadget {
            buffer,
            ty: desc.ty,
        };

        self.gadgets.insert(String::from(desc.label), gadget);

        let group_index = desc.index.group_index;

        while self.indices.len() <= group_index {
            self.indices.push(None);
        }

        if self.indices[group_index].is_none() {
            self.indices[group_index] = Some(HashMap::new());
        }

        // SATFTY: Checked upon
        let group_desc = self.indices[group_index].as_mut().unwrap();
        group_desc.insert(desc.index.binding_index, String::from(desc.label));

        self.needs_update = true;
    }

    pub fn acquire_gadget_mut(&mut self, label: &str) -> Result<&mut Gadget, ConveyorError> {
        self.gadgets
            .get_mut(label)
            .ok_or(ConveyorError::UnknownGadgetLabel)
    }

    pub fn update_gadget(
        &mut self,
        queue: &wgpu::Queue,
        gadget_label: &str,
        data: &[u8],
    ) -> Result<(), ConveyorError> {
        let gadget = self
            .gadgets
            .get(gadget_label)
            .ok_or(ConveyorError::UnknownGadgetLabel)?;

        queue.write_buffer(&gadget.buffer, 0, data);

        Ok(())
    }

    pub fn update_bundles(&mut self, device: &wgpu::Device, visibility: wgpu::ShaderStages) {
        self.bundles = Vec::new();

        for (group_index, group_desc) in self.indices.iter().enumerate() {
            if group_desc.is_none() {
                self.bundles.push(None);
                continue;
            }

            let group_desc = group_desc.as_ref().unwrap();

            let mut bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = Vec::new();
            let mut bind_group_entries: Vec<wgpu::BindGroupEntry> = Vec::new();

            for (binding_index, gadget_label) in group_desc {
                let gadget = self.gadgets.get(gadget_label).unwrap();

                bind_group_layout_entries.push(wgpu::BindGroupLayoutEntry {
                    binding: *binding_index,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: gadget.ty,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                });

                bind_group_entries.push(wgpu::BindGroupEntry {
                    binding: *binding_index,
                    resource: gadget.buffer.as_entire_binding(),
                })
            }

            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(&format!(
                        "Mraphics bind group layout with index {}",
                        group_index
                    )),
                    entries: &bind_group_layout_entries,
                });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&format!("Mraphics bind group with index {}", group_index)),
                layout: &bind_group_layout,
                entries: &bind_group_entries,
            });

            let bundle = Bundle {
                bind_group: bind_group,
                bind_group_layout: bind_group_layout,
            };

            self.bundles.push(Some(bundle));

            self.needs_update = false;
        }
    }

    pub fn attach_render_bundles(&self, render_pass: &mut wgpu::RenderPass) {
        for (index, maybe_bundle) in self.bundles.iter().enumerate() {
            if let Some(bundle) = maybe_bundle {
                render_pass.set_bind_group(index as u32, &bundle.bind_group, &[]);
            }
        }
    }

    pub fn attach_compute_bundles(&self, compute_pass: &mut wgpu::ComputePass) {
        for (index, maybe_bundle) in self.bundles.iter().enumerate() {
            if let Some(bundle) = maybe_bundle {
                compute_pass.set_bind_group(index as u32, &bundle.bind_group, &[]);
            }
        }
    }

    /// Collects bind group layouts from a collection of bundles.
    ///
    /// # Behavior
    /// - If no bundle defines a bind group at index `n`, but a later index `m > n` is defined,
    ///   the result at position `n` will be `None`.
    /// - If multiple bundles define a bind group at the same index, only the first
    ///   encountered (in iteration order) will be used.
    /// - The output length equals the maximum bundle length across the collection.
    pub fn collect_bind_group_layouts(
        bundles_collection: Vec<&Vec<Option<Bundle>>>,
    ) -> Vec<Option<&wgpu::BindGroupLayout>> {
        let mut max_len = 0;
        let mut bind_group_layouts = Vec::new();

        for bundles in bundles_collection.iter() {
            if bundles.len() > max_len {
                max_len = bundles.len()
            }
        }

        'outer: for i in 0..max_len {
            for bundles in bundles_collection.iter() {
                if !bundles.get(i).is_none() && !bundles[i].is_none() {
                    bind_group_layouts.push(Some(&bundles[i].as_ref().unwrap().bind_group_layout));
                    continue 'outer;
                }
            }

            bind_group_layouts.push(None);
        }

        bind_group_layouts
    }
}
