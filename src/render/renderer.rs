use crate::{
    Scene,
    geometry::Mesh,
    math::Camera,
    render::{Conveyor, ConveyorManager, PipelineManager, conveyor::GadgetDescriptor},
};

use crate::constants::{
    MODEL_MAT_INDEX, MODEL_MAT_LABEL, PROJECTION_MAT_INDEX, PROJECTION_MAT_LABEL, VIEW_MAT_INDEX,
    VIEW_MAT_LABEL,
};

pub struct Renderer<'window> {
    pub surface: wgpu::Surface<'window>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub clear_color: [f64; 4],

    pipeline_manager: PipelineManager,
    conveyor_manager: ConveyorManager,
    shared_conveyor: Conveyor,
}

impl<'window> Renderer<'window> {
    pub fn new(
        surface: wgpu::Surface<'window>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        adapter: &wgpu::Adapter,
    ) -> Self {
        let surface_caps = surface.get_capabilities(adapter);
        let surface_config = wgpu::SurfaceConfiguration {
            width: 100,
            height: 100,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

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

        shared_conveyor.upsert_gadget(
            &device,
            &GadgetDescriptor {
                label: MODEL_MAT_LABEL,
                index: MODEL_MAT_INDEX,
                size: 4 * 4 * 4,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                ty: wgpu::BufferBindingType::Uniform,
            },
        );

        Self {
            surface,
            surface_config,
            device,
            queue,
            clear_color: [0., 0., 0., 1.],
            pipeline_manager: PipelineManager::new(),
            conveyor_manager: ConveyorManager::new(),
            shared_conveyor,
        }
    }

    pub fn render<C: Camera>(
        &mut self,
        scene: &mut Scene,
        camera: &C,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

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
                        r: self.clear_color[0],
                        g: self.clear_color[1],
                        b: self.clear_color[2],
                        a: self.clear_color[3],
                    }),
                    store: wgpu::StoreOp::Store,
                },
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

        scene.traverse_mut(&mut |mesh: &mut Mesh| {
            self.render_mesh(&mut render_pass, mesh);
        });

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();

        Ok(())
    }

    pub fn render_mesh(&mut self, render_pass: &mut wgpu::RenderPass, mesh: &mut Mesh) {
        // SAFETY: initialized this gadget in Renderer::new()
        self.shared_conveyor
            .update_gadget(
                &self.queue,
                MODEL_MAT_LABEL,
                bytemuck::cast_slice(mesh.matrix().as_slice()),
            )
            .unwrap();

        let attr_conveyor = self
            .conveyor_manager
            .acquire_attr_conveyor(mesh.geometry.identifier());

        for attr in mesh.geometry.attributes_mut() {
            if attr.needs_update_buffer {
                attr_conveyor.upsert_gadget(
                    &self.device,
                    &GadgetDescriptor {
                        label: &attr.label,
                        index: attr.index,
                        size: attr.data.len() as u64,
                        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                    },
                );

                attr.needs_update_buffer = false;
            }

            if !attr.needs_update_value {
                continue;
            }

            // SAFETY: This may panic, but it's developer's responsibility
            attr_conveyor
                .update_gadget(&self.queue, &attr.label, &attr.data)
                .unwrap();

            attr.needs_update_value = false;
        }

        let needs_update = self.shared_conveyor.needs_update || attr_conveyor.needs_update;
        if needs_update {
            self.shared_conveyor.update_bundles(&self.device);
            attr_conveyor.update_bundles(&self.device);
        }

        let pipeline = self.pipeline_manager.acquire_pipeline(
            &self.device,
            self.surface_config.format,
            mesh.material.as_ref(),
            &Conveyor::collect_bind_group_layouts(vec![
                &self.shared_conveyor.bundles,
                &attr_conveyor.bundles,
            ]),
            needs_update,
        );

        self.shared_conveyor.attach_bundles(render_pass);
        attr_conveyor.attach_bundles(render_pass);

        render_pass.set_pipeline(pipeline);
        render_pass.draw(0..mesh.geometry.indices(), 0..1);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);
    }
}
