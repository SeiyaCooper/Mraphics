use crate::render::GadgetIndex;
use std::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

#[derive(Clone, Debug)]
pub struct Attribute<'a> {
    pub label: &'a str,
    pub index: GadgetIndex,
    pub data: Vec<u8>,
    pub needs_update_value: bool,
    pub needs_update_buffer: bool,
}

pub trait GeometryView<'a> {
    fn attributes(&self) -> &Vec<Attribute<'a>>;
    fn attributes_mut(&mut self) -> &mut Vec<Attribute<'a>>;
    fn indices(&self) -> u32;
    fn identifier(&self) -> &str;
}

static GLOBAL_GEOMETRY_ID: AtomicUsize = AtomicUsize::new(0);
const GEOMETRY_IDENTIFIER_PREFIX: &'static str = "mraphics-geometry-";

pub struct Geometry {
    pub attributes: Vec<Attribute<'static>>,

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

impl GeometryView<'static> for Geometry {
    fn attributes(&self) -> &Vec<Attribute<'static>> {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Vec<Attribute<'static>> {
        &mut self.attributes
    }

    fn indices(&self) -> u32 {
        self.attributes[0].data.len() as u32 / 4
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }
}
