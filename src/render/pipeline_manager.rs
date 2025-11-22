use crate::material::Material;
use std::collections::HashMap;

pub struct PipelineManager {
    pub pipeline_pool: HashMap<String, wgpu::RenderPipeline>,
}

impl PipelineManager {
    pub fn new() -> Self {
        Self {
            pipeline_pool: HashMap::new(),
        }
    }

    pub fn acquire_pipeline(
        &mut self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        material: &dyn Material,
        bind_groups: &[&wgpu::BindGroupLayout],
        force_update: bool,
    ) -> &wgpu::RenderPipeline {
        let pipeline_identifier = material.identifier();

        if !self.pipeline_pool.contains_key(pipeline_identifier) || force_update {
            let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Mraphics Shader"),
                source: wgpu::ShaderSource::Wgsl(material.shader_code().into()),
            });

            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Mraphics Render Pipeline Layout"),
                    bind_group_layouts: bind_groups,
                    push_constant_ranges: &[],
                });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Mraphics Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: Some("vs"),
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: Some("fs"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: texture_format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

            self.pipeline_pool
                .insert(String::from(material.identifier()), render_pipeline);
        }

        // SAFETY: Checked upon
        self.pipeline_pool.get(pipeline_identifier).unwrap()
    }
}
