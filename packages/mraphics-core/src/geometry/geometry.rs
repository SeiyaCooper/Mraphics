use crate::GadgetData;

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

#[derive(Debug)]
pub struct GeometryView {
    pub attributes: Vec<GadgetData>,
    pub uniforms: Vec<GadgetData>,
    pub indices: GeometryIndices,
}

impl GeometryView {
    pub fn new() -> Self {
        Self {
            indices: GeometryIndices::Sequential(0),
            attributes: Vec::new(),
            uniforms: Vec::new(),
        }
    }

    pub fn with_indices(mut self, indices: GeometryIndices) -> Self {
        self.indices = indices;
        self
    }

    pub fn with_attributes(mut self, attributes: Vec<GadgetData>) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn with_uniforms(mut self, uniforms: Vec<GadgetData>) -> Self {
        self.uniforms = uniforms;
        self
    }

    pub fn reset_vertices(&mut self) {
        self.attributes = Vec::new();
        self.indices = GeometryIndices::Sequential(0);
    }

    pub fn reset_uniforms(&mut self) {
        self.uniforms = Vec::new();
    }
}

pub trait Geometry {
    fn update_view(&self, view: &mut GeometryView);
}
