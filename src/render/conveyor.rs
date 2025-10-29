use std::collections::HashMap;

struct Gadget {
    buffer: wgpu::Buffer,
    ty: wgpu::BufferBindingType,
}

#[derive(Clone, Copy)]
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

struct Bundle {
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl Bundle {
    fn empty(device: &wgpu::Device) -> Self {
        let empty_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        });
        Self {
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                entries: &[],
                layout: &empty_layout,
            }),
            bind_group_layout: empty_layout,
        }
    }
}

#[derive(Debug)]
pub enum ConveyorError {
    UnknownGadgetLabel,
}

pub struct Conveyor<'a> {
    pub needs_update: bool,

    gadgets: HashMap<&'a str, Gadget>,
    bundles: Vec<Bundle>,
    indices: Vec<Option<HashMap<u32, &'a str>>>,
}

impl<'a> Conveyor<'a> {
    pub fn new() -> Self {
        Self {
            gadgets: HashMap::new(),
            bundles: Vec::new(),
            indices: Vec::new(),
            needs_update: false,
        }
    }

    pub fn init_gadget(&mut self, device: &wgpu::Device, desc: &GadgetDescriptor<'a>) {
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

        self.gadgets.insert(desc.label, gadget);

        let group_index = desc.index.group_index;

        while self.indices.len() <= group_index {
            self.indices.push(None);
        }

        if self.indices[group_index].is_none() {
            self.indices[group_index] = Some(HashMap::new());
        }

        // SATFTY: Checked upon
        let group_desc = self.indices[group_index].as_mut().unwrap();
        group_desc.insert(desc.index.binding_index, desc.label);

        self.needs_update = true;
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

    pub fn update_bundles(&mut self, device: &wgpu::Device) {
        self.bundles = Vec::new();

        for (group_index, group_desc) in self.indices.iter().enumerate() {
            let mut bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = Vec::new();
            let mut bind_group_entries: Vec<wgpu::BindGroupEntry> = Vec::new();

            if group_desc.is_none() {
                self.bundles.push(Bundle::empty(device));
                continue;
            }

            let group_desc = group_desc.as_ref().unwrap();

            for (binding_index, gadget_label) in group_desc {
                let gadget = self.gadgets.get(gadget_label).unwrap();

                bind_group_layout_entries.push(wgpu::BindGroupLayoutEntry {
                    binding: *binding_index,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, // Hard coded currently
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

            self.bundles.push(bundle);

            self.needs_update = false;
        }
    }

    pub fn attach_bundles(&self, render_pass: &mut wgpu::RenderPass) {
        for (index, bundle) in self.bundles.iter().enumerate() {
            render_pass.set_bind_group(index as u32, &bundle.bind_group, &[]);
        }
    }

    pub fn collect_bind_group_layouts(&mut self) -> Vec<&wgpu::BindGroupLayout> {
        self.bundles
            .iter()
            .map(|bundle| &bundle.bind_group_layout)
            .collect()
    }
}
