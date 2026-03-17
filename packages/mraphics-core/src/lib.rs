use std::{ops::Deref, sync::atomic::AtomicUsize};

static GLOBAL_ID_POOL: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MraphicsID {
    value: usize,
}

impl MraphicsID {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn acquire() -> Self {
        Self::new(GLOBAL_ID_POOL.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

impl Deref for MraphicsID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// Re-exports
mod scene;

pub use scene::*;

mod mesh_pool;
pub use mesh_pool::*;

mod render;
pub use render::*;

mod geometry;
pub use geometry::*;

mod material;
pub use material::*;

mod math;
pub use math::*;

mod animation;
pub use animation::*;

mod traits;
pub use traits::*;

pub mod constant;
