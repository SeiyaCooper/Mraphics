use crate::{Color, GadgetData, GadgetIndex, Material};

pub const COLOR_UNIFORM_LABEL: &'static str = "mraphics-color-uniform";
pub const COLOR_UNIFORM_INDEX: GadgetIndex = GadgetIndex {
    group_index: 1,
    binding_index: 0,
};

pub struct BasicMaterial {
    pub color: Color<f32>,
}

impl BasicMaterial {
    pub fn new() -> Self {
        Self {
            color: Color::cast_unchecked(&crate::constants::SEIYA_PINK),
        }
    }

    pub fn with_color(mut self, new_color: &Color<f32>) -> Self {
        self.color = new_color.clone();
        self
    }
}

impl Material for BasicMaterial {
    fn identifier(&self) -> &str {
        "Mraphics Basic Materil"
    }

    fn shader_code(&self) -> &str {
        include_str!("shaders/basic.wgsl")
    }

    fn update_view(&self, view: &mut super::MaterialView) {
        view.reset();

        view.uniforms.push(GadgetData {
            label: COLOR_UNIFORM_LABEL.to_string(),
            index: COLOR_UNIFORM_INDEX,
            data: bytemuck::cast_slice::<f32, u8>(&self.color).to_vec(),
            needs_update_value: true,
            needs_update_buffer: true,
        });
    }
}
