use crate::geometry::Mesh;

pub struct Scene<'attr> {
    pub meshes: Vec<Mesh<'attr>>,
}

impl<'attr> Scene<'attr> {
    pub fn new() -> Self {
        Self { meshes: Vec::new() }
    }

    pub fn add_mesh(&mut self, mesh: Mesh<'attr>) -> usize {
        self.meshes.push(mesh);
        self.meshes.len() - 1
    }

    pub fn traverse<F: Fn(&Mesh<'attr>)>(&self, callback: &F) {
        for mesh in &self.meshes {
            mesh.traverse(callback);
        }
    }

    pub fn traverse_mut<F: FnMut(&mut Mesh<'attr>)>(&mut self, callback: &mut F) {
        for mesh in &mut self.meshes {
            mesh.traverse_mut(callback);
        }
    }
}
