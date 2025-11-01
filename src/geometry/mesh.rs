use crate::{geometry::GeometryView, material::Material, math::Matrix};

pub struct Mesh<'attr> {
    pub children: Vec<Mesh<'attr>>,
    pub geometry: Box<dyn GeometryView<'attr>>,
    pub material: Box<dyn Material>,
    pub matrix: Matrix<f32>,
}

impl<'attr> Mesh<'attr> {
    pub fn new<G: GeometryView<'attr> + 'static, M: Material + 'static>(
        geometry: G,
        material: M,
    ) -> Self {
        Self {
            children: Vec::new(),
            geometry: Box::new(geometry),
            material: Box::new(material),
            matrix: Matrix::identity(4),
        }
    }

    pub fn add_child(&mut self, child: Mesh<'attr>) {
        self.children.push(child);
    }

    pub fn traverse<F: Fn(&Mesh<'attr>)>(&self, callback: &F) {
        callback(self);

        for child in &self.children {
            child.traverse(callback);
        }
    }

    pub fn traverse_mut<F: FnMut(&mut Mesh<'attr>)>(&mut self, callback: &mut F) {
        callback(self);

        for child in &mut self.children {
            child.traverse_mut(callback);
        }
    }
}
