#[derive(Clone, Debug)]
pub struct Attribute {
    pub index: (usize, usize),
    pub data: Vec<f64>,
}

pub struct Geometry {
    pub attributes: Vec<Attribute>,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }
}
