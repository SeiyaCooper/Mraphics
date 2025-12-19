use crate::{Geometry, Material, RenderInstance, Renderable};
use std::sync::atomic::AtomicUsize;

static GLOBAL_MESH_ID: AtomicUsize = AtomicUsize::new(0);

pub trait MeshLike<G: Geometry, M: Material>: Renderable {
    fn geometry(&self) -> &G;
    fn material(&self) -> &M;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MeshIndex(usize);

impl MeshIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

pub struct Mesh<G: Geometry, M: Material> {
    pub identifier: MeshIndex,
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

    pub fn acquire_id() -> MeshIndex {
        MeshIndex::new(GLOBAL_MESH_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

impl<G: Geometry, M: Material> Renderable for Mesh<G, M> {
    fn identifier(&self) -> usize {
        self.identifier.index()
    }

    fn build_instance(&self) -> RenderInstance {
        RenderInstance::new(self.identifier.index().to_string(), &self.material)
    }
}

impl<G: Geometry, M: Material> MeshLike<G, M> for Mesh<G, M> {
    fn geometry(&self) -> &G {
        &self.geometry
    }

    fn material(&self) -> &M {
        &self.material
    }
}
