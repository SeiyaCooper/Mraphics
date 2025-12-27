use crate::{Scene, animation::Action};
use std::{cell::RefCell, rc::Rc};

pub trait Animation<'res> {
    fn into_action(self, scene: Rc<RefCell<Scene>>) -> Action<'res>;
}
