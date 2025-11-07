use crate::{geometry::GeometryView, material::Material};
use nalgebra::Matrix4;

pub struct Mesh {
    pub children: Vec<Mesh>,
    pub geometry: Box<dyn GeometryView>,
    pub material: Box<dyn Material>,
    pub matrix: Matrix4<f32>,
}

impl Mesh {
    pub fn new<G: GeometryView + 'static, M: Material + 'static>(geometry: G, material: M) -> Self {
        Self {
            children: Vec::new(),
            geometry: Box::new(geometry),
            material: Box::new(material),
            matrix: Matrix4::identity(),
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
