use crate::Node;

pub struct Scene {
    pub node: Node,
}

impl Scene {
    pub fn new() -> Self {
        Self { node: Node::new() }
    }
}
