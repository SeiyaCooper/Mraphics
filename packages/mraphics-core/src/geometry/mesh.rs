use crate::{Geometry, InstanceUpdater, Material, RenderInstance};
use std::{marker::PhantomData, sync::atomic::AtomicUsize};

static GLOBAL_MESH_ID: AtomicUsize = AtomicUsize::new(0);

pub trait MeshLike: InstanceUpdater {
    /// Returns the unique identifier of this mesh.
    fn identifier(&self) -> usize;

    /// Builds a [`RenderInstance`] using this mesh's data.
    fn build_instance(&self) -> RenderInstance;

    /// Updates self before updating the render instance, optional.
    fn update(&mut self) {}
}

#[derive(Clone)]
pub struct MeshHandle<M: MeshLike> {
    pub id: usize,
    _marker: PhantomData<M>,
}

impl<M: MeshLike> MeshHandle<M> {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

pub struct Mesh<G: Geometry, M: Material> {
    pub identifier: usize,
    pub geometry: G,
    pub material: M,
}

impl<G: Geometry, M: Material> Mesh<G, M> {
    pub fn new(geometry: G, material: M) -> Self {
        Self {
            identifier: Self::acquire_id(),
            geometry,
            material,
        }
    }

    pub fn acquire_id() -> usize {
        GLOBAL_MESH_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

impl<G: Geometry, M: Material> InstanceUpdater for Mesh<G, M> {
    fn update_instance(&self, instance: &mut RenderInstance) {
        self.geometry.update_view(&mut instance.geometry);
        self.material.update_view(&mut instance.material);
    }
}

impl<G: Geometry, M: Material> MeshLike for Mesh<G, M> {
    fn identifier(&self) -> usize {
        self.identifier
    }

    fn build_instance(&self) -> RenderInstance {
        let mut instance = RenderInstance::new(self.identifier, &self.material);

        self.geometry.init_view(&mut instance.geometry);
        self.update_instance(&mut instance);

        instance
    }

    fn update(&mut self) {
        self.geometry.update();
    }
}
