use std::{any::Any, collections::HashMap};

use crate::{MeshHandle, MeshLike};

pub struct MeshPool {
    pub meshes: Vec<Box<dyn Any>>,
    mesh_map: HashMap<usize, usize>,
}

impl MeshPool {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            mesh_map: HashMap::new(),
        }
    }

    pub fn add_mesh<M: MeshLike + 'static>(&mut self, mesh: M) -> MeshHandle<M> {
        let id = mesh.identifier();

        self.meshes.push(Box::new(mesh));
        self.mesh_map.insert(id, self.meshes.len() - 1);

        MeshHandle::<M>::new(id)
    }

    pub fn acquire_mesh_mut_unchecked<M: MeshLike + 'static>(&mut self, id: usize) -> &mut M {
        self.meshes[*self.mesh_map.get(&id).unwrap()]
            .downcast_mut::<M>()
            .unwrap()
    }
}
