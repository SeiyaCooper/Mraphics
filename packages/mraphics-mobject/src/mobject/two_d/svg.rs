use mraphics_core::{InstanceUpdater, MeshLike, MraphicsID};

pub struct SVGMobj {
    identifier: MraphicsID,

    pub code: String,
}

impl SVGMobj {
    pub fn new(code: String) -> Self {
        Self {
            identifier: MraphicsID::acquire(),

            code,
        }
    }
}

impl InstanceUpdater for SVGMobj {
    fn update_instance(&self, instance: &mut mraphics_core::RenderInstance) {
        todo!()
    }
}

impl MeshLike for SVGMobj {
    fn identifier(&self) -> MraphicsID {
        self.identifier
    }

    fn build_instance(&self) -> mraphics_core::RenderInstance {
        todo!()
    }

    fn update(&mut self) {}
}
