use std::collections::HashMap;
use wgpu::BindGroupLayoutDescriptor;

struct Gadget {
    label: &'static str,
    index: GadgetIndex,
    buffer: wgpu::Buffer,
}

#[derive(Clone, Copy)]
pub struct GadgetIndex {
    pub group_index: usize,
    pub binding_index: u32,
}

pub struct GadgetDescriptor {
    pub label: &'static str,
    pub index: GadgetIndex,
    pub size: u64,
    pub usage: wgpu::BufferUsages,
}

#[derive(Debug)]
pub enum ConveyorError {
    UnknownGadgetLabel,
}

pub struct Conveyor {
    gadgets: HashMap<String, Gadget>,
    bind_group_layout_desc: BindGroupLayoutDescriptor<'static>,
}

impl Conveyor {
    pub fn new() -> Self {
        let bind_group_layout_desc = wgpu::BindGroupLayoutDescriptor {
            entries: &[],
            label: Some("Mraphics Bind Group Layout"),
        };

        Self {
            gadgets: HashMap::new(),
            bind_group_layout_desc,
        }
    }

    pub fn init_gadget(&mut self, device: &wgpu::Device, desc: &GadgetDescriptor) {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(desc.label),
            size: desc.size,
            usage: desc.usage,
            mapped_at_creation: false,
        });

        let gadget = Gadget {
            label: desc.label,
            index: desc.index,
            buffer,
        };

        self.gadgets.insert(String::from(desc.label), gadget);
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

    fn update_bind_group_layout(&mut self) {
        todo!();
    }
}
