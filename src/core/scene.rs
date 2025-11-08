use crate::geometry::Mesh;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

static GLOBAL_MESH_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Scene {
    pub meshes: HashMap<usize, Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> usize {
        self.meshes
            .insert(GLOBAL_MESH_ID.fetch_add(1, Relaxed), mesh);
        self.meshes.len() - 1
    }

    pub fn get_mesh(&self, index: usize) -> &Mesh {
        self.meshes.get(&index).unwrap()
    }

    pub fn get_mesh_mut(&mut self, index: usize) -> &mut Mesh {
        self.meshes.get_mut(&index).unwrap()
    }

    pub fn traverse<F: Fn(&Mesh)>(&self, callback: &F) {
        for (_, mesh) in &self.meshes {
            mesh.traverse(callback);
        }
    }

    pub fn traverse_mut<F: FnMut(&mut Mesh)>(&mut self, callback: &mut F) {
        for (_, mesh) in &mut self.meshes {
            mesh.traverse_mut(callback);
        }
    }
}
