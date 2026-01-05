use crate::{MeshPool, Scene, animation::Action};
use std::{cell::RefCell, rc::Rc};

pub trait Animation<'res> {
    fn into_action(
        self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'res>;
}

pub trait AsIntermediate {
    type Intermediate;

    fn as_intermediate(&self) -> Self::Intermediate;
}
