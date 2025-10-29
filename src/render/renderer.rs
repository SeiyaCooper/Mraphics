use crate::{
    Scene,
    geometry::Mesh,
    math::Camera,
    render::{
        Conveyor, PipelineManager,
        conveyor::{GadgetDescriptor, GadgetIndex},
    },
};

pub const VIEW_MAT_LABEL: &'static str = "mraphics-view-mat";
pub const PROJECTION_MAT_LABEL: &'static str = "mraphics-projection-mat";
pub const MODEL_MAT_LABEL: &'static str = "mraphics-model-mat";

pub struct Renderer<'window> {
    pub surface: wgpu::Surface<'window>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub clear_color: [f64; 4],

    pipeline_manager: PipelineManager,
    conveyor: Conveyor<'window>,
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

        let mut conveyor = Conveyor::new();
        conveyor.init_gadget(
            &device,
            &GadgetDescriptor {
                label: VIEW_MAT_LABEL,
                index: GadgetIndex {
                    group_index: 0,
                    binding_index: 0,
                },
                size: 4 * 4 * 4,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                ty: wgpu::BufferBindingType::Uniform,
            },
        );
        conveyor.init_gadget(
            &device,
            &GadgetDescriptor {
                label: PROJECTION_MAT_LABEL,
                index: GadgetIndex {
                    group_index: 0,
                    binding_index: 1,
                },
                size: 4 * 4 * 4,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                ty: wgpu::BufferBindingType::Uniform,
            },
        );

        conveyor.init_gadget(
            &device,
            &GadgetDescriptor {
                label: MODEL_MAT_LABEL,
                index: GadgetIndex {
                    group_index: 0,
                    binding_index: 2,
                },
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
            conveyor,
        }
    }

    pub fn render(&mut self, scene: &mut Scene, camera: &Camera) -> Result<(), wgpu::SurfaceError> {
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

        scene.traverse_mut(&mut |mesh| {
            self.render_mesh(&mut render_pass, mesh, camera);
        });

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();

        Ok(())
    }

    pub fn render_mesh(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        mesh: &Mesh,
        camera: &Camera,
    ) {
        // SAFETY: initialized these gadgets in Renderer::new()
        self.conveyor
            .update_gadget(
                &self.queue,
                VIEW_MAT_LABEL,
                camera.view_mat.as_static::<4, 4>().as_bytes(),
            )
            .unwrap();
        self.conveyor
            .update_gadget(
                &self.queue,
                PROJECTION_MAT_LABEL,
                camera.projection_mat.as_static::<4, 4>().as_bytes(),
            )
            .unwrap();
        self.conveyor
            .update_gadget(
                &self.queue,
                MODEL_MAT_LABEL,
                mesh.matrix.as_static::<4, 4>().as_bytes(),
            )
            .unwrap();

        let needs_update = self.conveyor.needs_update;
        if needs_update {
            self.conveyor.update_bundles(&self.device);
        }

        let pipeline = self.pipeline_manager.acquire_pipeline(
            &self.device,
            self.surface_config.format,
            mesh.material.as_ref(),
            &self.conveyor.collect_bind_group_layouts(),
            needs_update,
        );

        self.conveyor.attach_bundles(render_pass);

        render_pass.set_pipeline(pipeline);
        render_pass.draw(0..3, 0..1);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);
    }
}
