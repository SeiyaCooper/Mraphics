use crate::material::Material;

pub struct BasicMaterial {}

impl Material for BasicMaterial {
    fn identifier(&self) -> &'static str {
        "Mraphics Basic Materil"
    }

    fn shader_code(&self) -> String {
        include_str!("shaders/basic.wgsl").to_string()
    }
}
