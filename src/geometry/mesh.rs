use crate::{geometry::Geometry, material::Material};

pub struct Mesh {
    pub children: Vec<Mesh>,
    pub geometry: Geometry,
    pub material: Box<dyn Material>,
}

impl Mesh {
    pub fn new<M: Material + 'static>(geometry: Geometry, material: M) -> Self {
        Self {
            children: Vec::new(),
            geometry,
            material: Box::new(material),
        }
    }

    pub fn add_child(&mut self, child: Mesh) {
        self.children.push(child);
    }

    pub fn traverse<F: Fn(&Mesh)>(&self, callback: &F) {
        callback(self);

        for child in &self.children {
            child.traverse(callback);
        }
    }

    pub fn traverse_mut<F: FnMut(&mut Mesh)>(&mut self, callback: &mut F) {
        callback(self);

        for child in &mut self.children {
            child.traverse_mut(callback);
        }
    }
}
