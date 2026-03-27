use crate::{Material, Pass};

#[derive(Clone)]
pub struct MultiColoredMaterial {}

impl MultiColoredMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for MultiColoredMaterial {
    fn identifier(&self) -> &str {
        "Mraphics Multi-colored Material"
    }

    fn init_view(&self, view: &mut super::MaterialView) {
        view.render_process
            .queue_pass(Pass::render(include_str!("shaders/multi_colored.wgsl")));
    }
}
