use crate::render::GadgetData;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

pub trait GeometryView {
    fn attributes(&self) -> &Vec<GadgetData>;
    fn attributes_mut(&mut self) -> &mut Vec<GadgetData>;
    fn indices(&self) -> u32;
    fn identifier(&self) -> &str;
}

static GLOBAL_GEOMETRY_ID: AtomicUsize = AtomicUsize::new(0);
const GEOMETRY_IDENTIFIER_PREFIX: &'static str = "mraphics-geometry-";

pub struct Geometry {
    pub attributes: Vec<GadgetData>,

    identifier: String,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
            identifier: String::from(GEOMETRY_IDENTIFIER_PREFIX)
                + &GLOBAL_GEOMETRY_ID.fetch_add(1, Relaxed).to_string(),
        }
    }

    pub fn with_id_prefix(prefix: String) -> Self {
        Self {
            attributes: Vec::new(),
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

    fn indices(&self) -> u32 {
        self.attributes[0].data.len() as u32 / 4
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

            fn indices(&self) -> u32 {
                self.inner.indices()
            }
        }
    };
}
