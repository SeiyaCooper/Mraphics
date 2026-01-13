use crate::{MeshPool, Scene, animation::Action};
use std::{cell::RefCell, rc::Rc};

pub trait Animation<'res> {
    fn into_action(
        self,
        mesh_pool: Rc<RefCell<MeshPool>>,
        scene: Rc<RefCell<Scene>>,
    ) -> Action<'res>;
}

/// A trait that specifies a struct can both:
/// - Generate an intermediate representation
/// - Update self from an intermediate representation
pub trait Representable {
    type Intermediate;

    fn as_intermediate(&self) -> Self::Intermediate;
    fn update_from_intermediate(&mut self, repr: &Self::Intermediate);
}
