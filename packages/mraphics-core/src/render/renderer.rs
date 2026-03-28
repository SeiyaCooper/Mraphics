use crate::constant::{
    INDEX_BUFFER_LABEL, PROJECTION_MAT_INDEX, PROJECTION_MAT_LABEL, VIEW_MAT_INDEX, VIEW_MAT_LABEL,
};
use crate::{
    Camera, Color, Conveyor, ConveyorManager, GadgetData, GadgetDescriptor, GeometryIndices, Pass,
    PassContext, PipelineManager, RenderInstance,
};
use wgpu::{
    CommandEncoder, ComputePassDescriptor, Texture, TextureFormat, TextureView, util::DeviceExt,
};

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

        let clear_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Mraphics Clear Pass"),
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

        drop(clear_pass);

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

        fn render_recursive(
            this: &mut Renderer,
            texture_format: TextureFormat,
            encoder: &mut CommandEncoder,
            instance: &mut RenderInstance,
            view: &TextureView,
        ) {
            if instance.visible {
                this.render_instance(texture_format, encoder, instance, view);
            }

            for mut child in &mut instance.children {
                render_recursive(this, texture_format, encoder, &mut child, view);
            }
        }

        for instance in instances {
            render_recursive(self, texture_format, &mut encoder, instance, &view);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
    }

    /// Renders a single instance by setting up its geometry and material data,
    /// updating necessary buffers, and issuing draw commands.
    ///
    /// # Arguments
    /// * `texture_format` - The format of the current render target
    /// * `render_pass` - The active render pass to record commands into
    /// * `instance` - The render instance to draw
    /// * `clear_color` - The color used to clear the screen
    /// * `view` - Texture view
    fn render_instance(
        &mut self,
        texture_format: TextureFormat,
        encoder: &mut CommandEncoder,
        instance: &mut RenderInstance,
        view: &TextureView,
    ) {
        // == Walk through the render process and execute all passes ==
        let passes = std::mem::take(&mut instance.material.render_process.passes);
        for (pass_index, pass) in passes.iter().enumerate() {
            match &pass.context {
                &PassContext::Render => {
                    self.execute_render_pass(
                        texture_format,
                        encoder,
                        instance,
                        view,
                        pass,
                        pass_index,
                    );
                }
                &PassContext::Compute { workgroup_size } => {
                    self.execute_compute_pass(encoder, instance, pass, pass_index, workgroup_size);
                }
            }
        }
        let _ = std::mem::replace(&mut instance.material.render_process.passes, passes);
    }

    fn execute_render_pass(
        &mut self,
        texture_format: TextureFormat,
        encoder: &mut CommandEncoder,
        instance: &mut RenderInstance,
        view: &TextureView,
        pass: &Pass,
        pass_index: usize,
    ) {
        // == Transmit data to corresponding buffers ==
        self.update_gadgets(
            instance, true, /* Storage buffers are read only to vertex shaders */
        );

        // == Collect bind groups ==
        let mesh_conveyor = self
            .mesh_conveyor_manager
            .acquire_conveyor(&instance.identifier);
        let material_conveyor = self
            .material_conveyor_manager
            .acquire_conveyor(&(instance.identifier.to_string() + &instance.material.identifier));

        let needs_update = self.shared_conveyor.needs_update
            || mesh_conveyor.needs_update
            || material_conveyor.needs_update;
        if needs_update {
            let visibility = wgpu::ShaderStages::VERTEX_FRAGMENT;
            self.shared_conveyor
                .update_bundles(&self.device, visibility);
            mesh_conveyor.update_bundles(&self.device, visibility);
            material_conveyor.update_bundles(&self.device, visibility);
        }

        let maybe_bind_group_layouts = Conveyor::collect_bind_group_layouts(vec![
            &self.shared_conveyor.bundles,
            &mesh_conveyor.bundles,
            &material_conveyor.bundles,
        ]);
        let bind_group_placeholder = if maybe_bind_group_layouts.contains(&None) {
            Some(
                self.device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some(&format!("Mraphics bind group layout placeholder",)),
                        entries: &[],
                    }),
            )
        } else {
            None
        };
        let bind_group_layouts = maybe_bind_group_layouts
            .iter()
            .map(|bind_group| {
                bind_group.unwrap_or_else(|| bind_group_placeholder.as_ref().unwrap())
            })
            .collect::<Vec<_>>();

        // == Create render pass and execute it ==
        let render_pipeline = self.pipeline_manager.acquire_render_pipeline(
            &self.device,
            texture_format,
            instance,
            pass,
            pass_index,
            &bind_group_layouts,
            needs_update,
        );

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Mraphics Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            ..Default::default()
        });

        self.shared_conveyor.attach_render_bundles(&mut render_pass);
        mesh_conveyor.attach_render_bundles(&mut render_pass);
        material_conveyor.attach_render_bundles(&mut render_pass);

        render_pass.set_pipeline(render_pipeline);

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
                render_pass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..(indices.data.len() as u32), 0, 0..1);
            }
        }
    }

    fn execute_compute_pass(
        &mut self,
        encoder: &mut CommandEncoder,
        instance: &mut RenderInstance,
        pass: &Pass,
        pass_index: usize,
        workgroup_size: (u32, u32, u32),
    ) {
        // == Transmit data to corresponding buffers ==
        self.update_gadgets(
            instance, false,
            /* Storage buffers are writable to compute shaders */
        );

        // == Collect bind groups ==
        let mesh_conveyor = self
            .mesh_conveyor_manager
            .acquire_conveyor(&instance.identifier);
        let material_conveyor = self
            .material_conveyor_manager
            .acquire_conveyor(&(instance.identifier.to_string() + &instance.material.identifier));

        let needs_update = self.shared_conveyor.needs_update
            || mesh_conveyor.needs_update
            || material_conveyor.needs_update;
        if needs_update {
            let visibility = wgpu::ShaderStages::COMPUTE;
            self.shared_conveyor
                .update_bundles(&self.device, visibility);
            mesh_conveyor.update_bundles(&self.device, visibility);
            material_conveyor.update_bundles(&self.device, visibility);
        }

        let maybe_bind_group_layouts = Conveyor::collect_bind_group_layouts(vec![
            &self.shared_conveyor.bundles,
            &mesh_conveyor.bundles,
            &material_conveyor.bundles,
        ]);
        let bind_group_placeholder = if maybe_bind_group_layouts.contains(&None) {
            Some(
                self.device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some(&format!("Mraphics bind group layout placeholder",)),
                        entries: &[],
                    }),
            )
        } else {
            None
        };
        let bind_group_layouts = maybe_bind_group_layouts
            .iter()
            .map(|bind_group| {
                bind_group.unwrap_or_else(|| bind_group_placeholder.as_ref().unwrap())
            })
            .collect::<Vec<_>>();

        let compute_pipeline = self.pipeline_manager.acquire_compute_pipeline(
            &self.device,
            instance,
            pass,
            pass_index,
            &bind_group_layouts,
            needs_update,
        );

        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("Mraphics Compute Pass"),
            timestamp_writes: None,
        });

        self.shared_conveyor
            .attach_compute_bundles(&mut compute_pass);
        mesh_conveyor.attach_compute_bundles(&mut compute_pass);
        material_conveyor.attach_compute_bundles(&mut compute_pass);

        compute_pass.set_pipeline(compute_pipeline);

        compute_pass.dispatch_workgroups(workgroup_size.0, workgroup_size.1, workgroup_size.2);
    }

    fn update_gadgets(&mut self, instance: &mut RenderInstance, read_only: bool) {
        let mesh_conveyor = self
            .mesh_conveyor_manager
            .acquire_conveyor(&instance.identifier);

        update_gadgets(
            &self.device,
            &self.queue,
            mesh_conveyor,
            &mut instance.geometry.storages,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Storage { read_only },
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
            &mut instance.material.storages,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Storage { read_only },
        );

        update_gadgets(
            &self.device,
            &self.queue,
            material_conveyor,
            &mut instance.material.uniforms,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            wgpu::BufferBindingType::Uniform,
        );
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

    // Set buffer type, which differs between render and compute passes:
    // - Render pass:  storage variables are read-only
    // - Compute pass: storage variables are writable
    //
    // SAFETY: `data.needs_update_buffer` is `true` when the gadget is uninitialized,
    // since we already checked it above, the gadget should have been initialized and `.unwrap()` is safe.
    if conveyor.acquire_gadget_mut(&data.label).unwrap().ty != ty {
        conveyor.acquire_gadget_mut(&data.label).unwrap().ty = ty;
        conveyor.needs_update = true;
    };

    if !data.needs_update_value {
        return;
    }

    // SAFETY: This may panic, but it's developer's responsibility
    conveyor
        .update_gadget(queue, &data.label, &data.data)
        .unwrap();

    data.needs_update_value = false;
}
