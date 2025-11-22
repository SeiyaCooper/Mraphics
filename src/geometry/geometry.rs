use crate::render::GadgetData;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

pub trait AllowedIndexFormat {}
impl AllowedIndexFormat for u32 {}
impl AllowedIndexFormat for u16 {}

#[derive(Debug, Clone)]
pub struct CustomIndices<T: AllowedIndexFormat> {
    pub data: Vec<T>,
    pub buffer: Option<wgpu::Buffer>,
}

impl<T: AllowedIndexFormat> CustomIndices<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data, buffer: None }
    }
}

#[derive(Debug, Clone)]
pub enum GeometryIndices {
    Sequential(u32),
    CustomU16(CustomIndices<u16>),
    CustomU32(CustomIndices<u32>),
}

pub trait GeometryView {
    fn attributes(&self) -> &Vec<GadgetData>;
    fn attributes_mut(&mut self) -> &mut Vec<GadgetData>;
    fn indices(&self) -> &GeometryIndices;
    fn indices_mut(&mut self) -> &mut GeometryIndices;
    fn identifier(&self) -> &str;
}

static GLOBAL_GEOMETRY_ID: AtomicUsize = AtomicUsize::new(0);
const GEOMETRY_IDENTIFIER_PREFIX: &'static str = "mraphics-geometry-";

pub struct Geometry {
    pub attributes: Vec<GadgetData>,
    pub indices: GeometryIndices,

    identifier: String,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
            indices: GeometryIndices::Sequential(0),
            identifier: String::from(GEOMETRY_IDENTIFIER_PREFIX)
                + &GLOBAL_GEOMETRY_ID.fetch_add(1, Relaxed).to_string(),
        }
    }

    pub fn with_id_prefix(prefix: String) -> Self {
        Self {
            attributes: Vec::new(),
            indices: GeometryIndices::Sequential(0),
            identifier: prefix + &GLOBAL_GEOMETRY_ID.fetch_add(1, Relaxed).to_string(),
        }
    }
}

impl GeometryView for Geometry {
    fn attributes(&self) -> &Vec<GadgetData> {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Vec<GadgetData> {
        &mut self.attributes
    }

    fn indices(&self) -> &GeometryIndices {
        &self.indices
    }

    fn indices_mut(&mut self) -> &mut GeometryIndices {
        &mut self.indices
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }
}

#[macro_export]
macro_rules! impl_inner_geometry_view {
    ($type:ty) => {
        impl $crate::geometry::GeometryView for $type {
            fn attributes(&self) -> &Vec<crate::render::GadgetData> {
                self.inner.attributes()
            }

            fn attributes_mut(&mut self) -> &mut Vec<crate::render::GadgetData> {
                self.inner.attributes_mut()
            }

            fn identifier(&self) -> &str {
                self.inner.identifier()
            }

            fn indices(&self) -> &crate::geometry::GeometryIndices {
                self.inner.indices()
            }

            fn indices_mut(&mut self) -> &mut crate::geometry::GeometryIndices {
                self.inner.indices_mut()
            }
        }
    };
}
