use crate::constants::{
    INDEX_BUFFER_LABEL, PROJECTION_MAT_INDEX, PROJECTION_MAT_LABEL, VIEW_MAT_INDEX, VIEW_MAT_LABEL,
};
use crate::{
    Camera, Color, Conveyor, ConveyorManager, GadgetData, GadgetDescriptor, GeometryIndices,
    PipelineManager, RenderInstance,
};
use wgpu::{Texture, TextureFormat, util::DeviceExt};

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pipeline_manager: PipelineManager,
    mesh_conveyor_manager: ConveyorManager<usize>,
    material_conveyor_manager: ConveyorManager<String>,
    shared_conveyor: Conveyor,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        let mut shared_conveyor = Conveyor::new();
        shared_conveyor.upsert_gadget(
            &device,
            &GadgetDescriptor {
                label: VIEW_MAT_LABEL,
                index: VIEW_MAT_INDEX,
                size: 4 * 4 * 4,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                ty: wgpu::BufferBindingType::Uniform,
            },
        );
        shared_conveyor.upsert_gadget(
            &device,
            &GadgetDescriptor {
                label: PROJECTION_MAT_LABEL,
                index: PROJECTION_MAT_INDEX,
                size: 4 * 4 * 4,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                ty: wgpu::BufferBindingType::Uniform,
            },
        );

        Self {
            device,
            queue,
            pipeline_manager: PipelineManager::new(),
            mesh_conveyor_manager: ConveyorManager::new(),
            material_conveyor_manager: ConveyorManager::new(),
            shared_conveyor,
        }
    }

    /// Reads RGBA pixel data from a GPU texture into a CPU-accessible byte vector.
    ///
    /// # Arguments
    /// * `texture` - The texture to read from
    /// * `size` - The (width, height) dimensions of the texture
    ///
    /// # Returns
    /// A Vec<u8> containing RGBA pixel data in row-major order
    pub fn read_texture_rgbau8(&self, texture: &Texture, size: (u32, u32)) -> Vec<u8> {
        let (width, height) = size;
        let unpadded_bytes_per_row = width * 4; // rgba
        let bytes_per_row =
            wgpu::util::align_to(unpadded_bytes_per_row, wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
        let buffer_size = (bytes_per_row * height) as wgpu::BufferAddress;
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Mraphics Texture Mapping Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfoBase {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfoBase {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(std::iter::once(encoder.finish()));

        let (sender, receiver) = std::sync::mpsc::sync_channel(1);

        buffer.map_async(wgpu::MapMode::Read, .., move |result| {
            sender.send(result).unwrap();
        });

        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();

        receiver.recv().unwrap().unwrap();

        let raw_data = buffer.get_mapped_range(..);
        let mut data = Vec::with_capacity((unpadded_bytes_per_row * height) as usize);

        for row in 0..height {
            let row_start = (row * bytes_per_row) as usize;
            let row_end = row_start + unpadded_bytes_per_row as usize;

            data.extend_from_slice(&raw_data[row_start..row_end]);
        }

        drop(raw_data);

        buffer.unmap();

        data
    }

    /// Renders a collection of instances to a texture using the specified camera and clear color.
    /// Updates shared camera matrices, processes each render instance, and submits commands to the GPU.
    ///
    /// # Arguments
    /// * `texture` - The target texture to render to
    /// * `texture_format` - The format of the target texture，used to build a render pipeline
    /// * `instances` - Mutable slice of render instances to draw
    /// * `camera` - Camera providing view and projection matrices
    /// * `clear_color` - Background color to clear the texture with
    pub fn render<C: Camera>(
        &mut self,
        texture: &Texture,
        texture_format: TextureFormat,
        instances: &mut [RenderInstance],
        camera: &C,
        clear_color: &Color<f64>,
    ) {
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Mraphics Command Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Mraphics Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: clear_color[0],
                        g: clear_color[1],
                        b: clear_color[2],
                        a: clear_color[3],
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            ..Default::default()
        });

        // SAFETY: initialized these gadgets in Renderer::new()
        self.shared_conveyor
            .update_gadget(&self.queue, VIEW_MAT_LABEL, camera.view_mat_data())
            .unwrap();
        self.shared_conveyor
            .update_gadget(
                &self.queue,
                PROJECTION_MAT_LABEL,
                camera.projection_mat_data(),
            )
            .unwrap();

        for instance in instances {
            self.render_instance(texture_format, &mut render_pass, instance);
        }

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
    }

    /// Renders a single instance by setting up its geometry and material data,
    /// updating necessary buffers, and issuing draw commands.
    ///
    /// # Arguments
    /// * `texture_format` - The format of the current render target
    /// * `render_pass` - The active render pass to record commands into
    /// * `instance` - The render instance to draw
    fn render_instance(
        &mut self,
        texture_format: TextureFormat,
        render_pass: &mut wgpu::RenderPass,
        instance: &mut RenderInstance,
    ) {
        let mesh_conveyor = self
            .mesh_conveyor_manager
            .acquire_conveyor(&instance.identifier);

        update_gadgets(
            &self.device,
            &self.queue,
            mesh_conveyor,
            &mut instance.geometry.attributes,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Storage { read_only: true },
        );

        update_gadgets(
            &self.device,
            &self.queue,
            mesh_conveyor,
            &mut instance.geometry.uniforms,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Uniform,
        );

        let material_conveyor = self
            .material_conveyor_manager
            .acquire_conveyor(&(instance.identifier.to_string() + &instance.material.identifier));

        update_gadgets(
            &self.device,
            &self.queue,
            material_conveyor,
            &mut instance.material.attributes,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Storage { read_only: true },
        );

        update_gadgets(
            &self.device,
            &self.queue,
            material_conveyor,
            &mut instance.material.uniforms,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Uniform,
        );

        let needs_update = self.shared_conveyor.needs_update
            || mesh_conveyor.needs_update
            || material_conveyor.needs_update;
        if needs_update {
            self.shared_conveyor.update_bundles(&self.device);
            mesh_conveyor.update_bundles(&self.device);
            material_conveyor.update_bundles(&self.device);
        }

        let pipeline = self.pipeline_manager.acquire_pipeline(
            &self.device,
            texture_format,
            &instance.material,
            &Conveyor::collect_bind_group_layouts(vec![
                &self.shared_conveyor.bundles,
                &mesh_conveyor.bundles,
                &material_conveyor.bundles,
            ]),
            needs_update,
        );

        self.shared_conveyor.attach_bundles(render_pass);
        mesh_conveyor.attach_bundles(render_pass);
        material_conveyor.attach_bundles(render_pass);

        render_pass.set_pipeline(pipeline);

        match &mut instance.geometry.indices {
            GeometryIndices::Sequential(indices) => {
                render_pass.draw(0..*indices, 0..1);
            }
            GeometryIndices::CustomU16(indices) => {
                if indices.buffer.is_none() {
                    indices.buffer.replace(self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(INDEX_BUFFER_LABEL),
                            contents: bytemuck::cast_slice(&indices.data),
                            usage: wgpu::BufferUsages::INDEX,
                        },
                    ));
                }

                // SAFETY: Checked upon
                let buffer = indices.buffer.as_ref().unwrap();
                render_pass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..(indices.data.len() as u32), 0, 0..1);
            }
            GeometryIndices::CustomU32(indices) => {
                if indices.buffer.is_none() {
                    if indices.buffer.is_none() {
                        indices.buffer.replace(self.device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some(INDEX_BUFFER_LABEL),
                                contents: bytemuck::cast_slice(&indices.data),
                                usage: wgpu::BufferUsages::INDEX,
                            },
                        ));
                    }
                }

                // SAFETY: Checked upon
                let buffer = indices.buffer.as_ref().unwrap();
                render_pass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..(indices.data.len() as u32), 0, 0..1);
            }
        }
    }
}

fn update_gadgets(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    conveyor: &mut Conveyor,
    gadget_data: &mut Vec<GadgetData>,
    usage: wgpu::BufferUsages,
    ty: wgpu::BufferBindingType,
) {
    for data in gadget_data {
        update_gadget(device, queue, conveyor, data, usage, ty);
    }
}

fn update_gadget(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    conveyor: &mut Conveyor,
    data: &mut GadgetData,
    usage: wgpu::BufferUsages,
    ty: wgpu::BufferBindingType,
) {
    if data.needs_update_buffer {
        conveyor.upsert_gadget(
            device,
            &GadgetDescriptor {
                label: &data.label,
                index: data.index,
                size: data.data.len() as u64,
                usage,
                ty,
            },
        );

        data.needs_update_buffer = false;
    }

    if !data.needs_update_value {
        return;
    }

    // SAFETY: This may panic, but it's developer's responsibility
    conveyor
        .update_gadget(queue, &data.label, &data.data)
        .unwrap();

    data.needs_update_value = false;
}
