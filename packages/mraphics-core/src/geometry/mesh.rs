use crate::{Geometry, Material, RenderInstance};
use std::{marker::PhantomData, sync::atomic::AtomicUsize};

static GLOBAL_MESH_ID: AtomicUsize = AtomicUsize::new(0);

pub trait MeshLike {
    /// Returns the unique identifier of this mesh.
    fn identifier(&self) -> usize;

    /// Build a render instance using this mesh's data.
    fn build_instance(&self) -> RenderInstance;

    /// Initializes self before building render instance, optional.
    fn init(&mut self) {}
}

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
            geometry: geometry,
            material: material,
        }
    }

    pub fn acquire_id() -> usize {
        GLOBAL_MESH_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

impl<G: Geometry, M: Material> MeshLike for Mesh<G, M> {
    fn identifier(&self) -> usize {
        self.identifier
    }

    fn build_instance(&self) -> RenderInstance {
        let mut instance = RenderInstance::new(self.identifier, &self.material);

        self.geometry.init_view(&mut instance.geometry);

        self.geometry.update_view(&mut instance.geometry);
        self.material.update_view(&mut instance.material);

        instance
    }

    fn init(&mut self) {
        self.geometry.init();
    }
}
