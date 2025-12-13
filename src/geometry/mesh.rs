use crate::{Geometry, Material, RenderInstance, Renderable};
use std::sync::atomic::AtomicUsize;

static GLOBAL_MESH_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Mesh<G: Geometry, M: Material> {
    pub identifier: usize,
    pub geometry: G,
    pub material: M,
}

impl<G: Geometry, M: Material> Mesh<G, M> {
    pub fn new(geometry: G, material: M) -> Self {
        Self {
            identifier: GLOBAL_MESH_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            geometry: geometry,
            material: material,
        }
    }
}

impl<G: Geometry, M: Material> Renderable for Mesh<G, M> {
    fn identifier(&self) -> usize {
        self.identifier
    }

    fn build_instance(&self) -> RenderInstance {
        RenderInstance::new(self.identifier.to_string(), &self.material)
    }
}
