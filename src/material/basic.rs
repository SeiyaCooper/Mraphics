use crate::{
    impl_material,
    material::Material,
    math::Color,
    render::{GadgetData, GadgetIndex},
};

pub const COLOR_UNIFORM_LABEL: &'static str = "mraphics-color-uniform";
pub const COLOR_UNIFORM_INDEX: GadgetIndex = GadgetIndex {
    group_index: 1,
    binding_index: 0,
};

pub struct BasicMaterial {
    pub uniforms: Vec<GadgetData>,
    pub attributes: Vec<GadgetData>,
}

impl BasicMaterial {
    pub fn new() -> Self {
        let mut uniforms = Vec::new();

        uniforms.push(GadgetData {
            label: COLOR_UNIFORM_LABEL.to_string(),
            index: COLOR_UNIFORM_INDEX,
            data: bytemuck::cast_slice::<f64, u8>(&vec![1.0, 1.0, 1.0, 1.0]).to_vec(),
            needs_update_value: true,
            needs_update_buffer: true,
        });

        Self {
            uniforms,
            attributes: Vec::new(),
        }
    }

    pub fn with_color(mut self, new_color: &Color<f32>) -> Self {
        self.uniforms[0].data = bytemuck::cast_slice::<f32, u8>(new_color).to_vec();
        self.uniforms[0].needs_update_value = true;
        self
    }
}

impl Material for BasicMaterial {
    fn identifier(&self) -> &'static str {
        "Mraphics Basic Materil"
    }

    fn shader_code(&self) -> String {
        include_str!("shaders/basic.wgsl").to_string()
    }
}

impl_material!(BasicMaterial);
