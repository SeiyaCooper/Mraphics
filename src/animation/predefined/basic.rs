use crate::{
    animation::{Action, Animation},
    geometry::Mesh,
};

pub struct MeshAnimation {
    pub mesh_index: usize,
    pub on_update: Box<dyn FnMut(&mut Mesh, f32, f32)>,
}

impl MeshAnimation {
    pub fn new(mesh_index: usize) -> Self {
        Self {
            mesh_index,
            on_update: Box::new(|_, _, _| {}),
        }
    }
}

impl Animation for MeshAnimation {
    fn into_action(mut self, scene: std::rc::Rc<std::cell::RefCell<crate::Scene>>) -> Action {
        let mut out = Action::new();

        out.on_update = Box::new(move |progress, elapsed_time| {
            (self.on_update)(
                scene.borrow_mut().get_mesh_mut(self.mesh_index),
                progress,
                elapsed_time,
            )
        });

        out
    }
}
