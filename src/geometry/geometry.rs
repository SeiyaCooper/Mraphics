use crate::Node;

#[derive(Clone, Debug)]
pub struct Attribute {
    pub data: Vec<f32>,
}

pub struct Geometry {
    pub node: Node,
    pub attributes: Vec<Attribute>,
}
