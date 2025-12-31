use wgpu::util::DeviceExt;

use crate::constants::{
    INDEX_BUFFER_LABEL, PROJECTION_MAT_INDEX, PROJECTION_MAT_LABEL, VIEW_MAT_INDEX, VIEW_MAT_LABEL,
};
use crate::{
    Camera, Color, Conveyor, ConveyorManager, GadgetData, GadgetDescriptor, GeometryIndices,
    PipelineManager, RenderInstance, Scene,
};

pub struct Renderer<'window> {
    pub surface: wgpu::Surface<'window>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pipeline_manager: PipelineManager,
    mesh_conveyor_manager: ConveyorManager<usize>,
    material_conveyor_manager: ConveyorManager<String>,
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
            format: wgpu::TextureFormat::Rgba8Unorm,
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

        Self {
            surface,
            surface_config,
            device,
            queue,
            pipeline_manager: PipelineManager::new(),
            mesh_conveyor_manager: ConveyorManager::new(),
            material_conveyor_manager: ConveyorManager::new(),
            shared_conveyor,
        }
    }

    pub fn render<C: Camera>(
        &mut self,
        scene: &mut Scene,
        camera: &C,
        clear_color: &Color<f64>,
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

        for instance in &mut scene.instances {
            self.render_instance(&mut render_pass, instance);
        }

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();

        Ok(())
    }

    pub fn render_instance(
        &mut self,
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
            self.surface_config.format,
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

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);
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
