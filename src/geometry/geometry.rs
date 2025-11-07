use crate::render::GadgetIndex;
use std::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

#[derive(Clone, Debug)]
pub struct Attribute {
    pub label: String,
    pub index: GadgetIndex,
    pub data: Vec<u8>,
    pub needs_update_value: bool,
    pub needs_update_buffer: bool,
}

pub trait GeometryView {
    fn attributes(&self) -> &Vec<Attribute>;
    fn attributes_mut(&mut self) -> &mut Vec<Attribute>;
    fn indices(&self) -> u32;
    fn identifier(&self) -> &str;
}

static GLOBAL_GEOMETRY_ID: AtomicUsize = AtomicUsize::new(0);
const GEOMETRY_IDENTIFIER_PREFIX: &'static str = "mraphics-geometry-";

pub struct Geometry {
    pub attributes: Vec<Attribute>,

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
}

impl GeometryView for Geometry {
    fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }

    fn indices(&self) -> u32 {
        self.attributes[0].data.len() as u32 / 4
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }
}
