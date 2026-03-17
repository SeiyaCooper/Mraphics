use mraphics_core::Material;

/// Material used for rendering [`Mobject2DStroke`] objects.
///
/// # Camera Compatibility
/// This material is specifically designed to work with [`crate::PerspectiveCamera`].
/// Using it with other camera types will result in incorrect rendering.
#[derive(Clone)]
pub struct Mobject2DMaterial {}

impl Mobject2DMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Mobject2DMaterial {
    fn identifier(&self) -> &str {
        "Mraphics Mobject2D Material"
    }

    fn shader_code(&self) -> &str {
        include_str!("shaders/mobject_2d.wgsl")
    }
}
