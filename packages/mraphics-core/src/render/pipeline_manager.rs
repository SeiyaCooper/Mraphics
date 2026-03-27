use wgpu::ComputePipelineDescriptor;

use crate::{Pass, RenderInstance, constant::PrimitiveTopology};
use std::collections::HashMap;

pub struct PipelineManager {
    pub render_pipelines: HashMap<String, wgpu::RenderPipeline>,
    pub compute_pipelines: HashMap<String, wgpu::ComputePipeline>,
}

impl PipelineManager {
    pub fn new() -> Self {
        Self {
            render_pipelines: HashMap::new(),
            compute_pipelines: HashMap::new(),
        }
    }

    pub fn acquire_render_pipeline(
        &mut self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        instance: &RenderInstance,
        render_pass: &Pass,
        pass_index: usize,
        bind_groups: &[&wgpu::BindGroupLayout],
        force_update: bool,
    ) -> &wgpu::RenderPipeline {
        let pipeline_identifier = format!(
            "{}{}{}",
            &instance.material.identifier,
            instance.topology.to_str(),
            pass_index
        );

        if !self.render_pipelines.contains_key(&pipeline_identifier) || force_update {
            self.insert_render_pipeline(
                device,
                texture_format,
                instance,
                render_pass,
                bind_groups,
                &pipeline_identifier,
            );
        }

        // SAFETY: Checked upon
        &self.render_pipelines.get(&pipeline_identifier).unwrap()
    }

    pub fn insert_render_pipeline(
        &mut self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        instance: &RenderInstance,
        render_pass: &Pass,
        bind_groups: &[&wgpu::BindGroupLayout],
        pipeline_identifier: &String,
    ) {
        self.render_pipelines.insert(
            String::from(pipeline_identifier),
            PipelineManager::build_render_pipeline(
                device,
                texture_format,
                render_pass,
                bind_groups,
                instance.topology,
            ),
        );
    }

    pub fn build_render_pipeline(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        render_pass: &Pass,
        bind_groups: &[&wgpu::BindGroupLayout],
        topology: PrimitiveTopology,
    ) -> wgpu::RenderPipeline {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Mraphics Shader"),
            source: wgpu::ShaderSource::Wgsl((&render_pass.shader_code).into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Mraphics Render Pipeline Layout"),
                bind_group_layouts: bind_groups,
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                topology: topology.to_wgpu(),
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
        })
    }

    pub fn acquire_compute_pipeline(
        &mut self,
        device: &wgpu::Device,
        instance: &RenderInstance,
        compute_pass: &Pass,
        pass_index: usize,
        bind_groups: &[&wgpu::BindGroupLayout],
        force_update: bool,
    ) -> &wgpu::ComputePipeline {
        let pipeline_identifier = format!(
            "{}{}{}",
            &instance.material.identifier,
            instance.topology.to_str(),
            pass_index
        );

        if !self.render_pipelines.contains_key(&pipeline_identifier) || force_update {
            self.insert_compute_pipeline(device, compute_pass, bind_groups, &pipeline_identifier);
        }

        // SAFETY: Checked upon
        &self.compute_pipelines.get(&pipeline_identifier).unwrap()
    }

    pub fn insert_compute_pipeline(
        &mut self,
        device: &wgpu::Device,
        compute_pass: &Pass,
        bind_groups: &[&wgpu::BindGroupLayout],
        pipeline_identifier: &String,
    ) {
        self.compute_pipelines.insert(
            String::from(pipeline_identifier),
            PipelineManager::build_compute_pipeline(device, compute_pass, bind_groups),
        );
    }

    pub fn build_compute_pipeline(
        device: &wgpu::Device,
        compute_pass: &Pass,
        bind_groups: &[&wgpu::BindGroupLayout],
    ) -> wgpu::ComputePipeline {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Mraphics Compute Shader"),
            source: wgpu::ShaderSource::Wgsl((&compute_pass.shader_code).into()),
        });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Mraphics Compute Pipeline Layout"),
                bind_group_layouts: bind_groups,
                push_constant_ranges: &[],
            });

        device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Mraphics Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader_module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        })
    }
}
