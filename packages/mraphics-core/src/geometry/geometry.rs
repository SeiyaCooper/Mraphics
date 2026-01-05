use crate::{GadgetData, GadgetIndex, constants};
use nalgebra::Matrix4;
use std::collections::HashMap;

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
pub enum GeometryViewError {
    UnknownAttributeLabel,
    UnknownUniformLabel,
}

#[derive(Debug)]
pub struct GeometryView {
    pub attributes: Vec<GadgetData>,
    attribute_map: HashMap<String, usize>,

    pub uniforms: Vec<GadgetData>,
    uniform_map: HashMap<String, usize>,

    pub indices: GeometryIndices,
}

impl GeometryView {
    pub fn new() -> Self {
        let mut out = Self {
            indices: GeometryIndices::Sequential(0),

            attributes: Vec::new(),
            attribute_map: HashMap::new(),

            uniforms: Vec::new(),
            uniform_map: HashMap::new(),
        };

        out.add_uniform(
            constants::MODEL_MAT_LABEL,
            constants::MODEL_MAT_INDEX,
            bytemuck::cast_slice(Matrix4::<f32>::identity().as_slice()).to_vec(),
        );

        out
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

    pub fn add_attribute(&mut self, label: &str, index: GadgetIndex, data: Vec<u8>) {
        let attribute = GadgetData {
            label: label.to_string(),
            index,
            data,
            needs_update_value: true,
            needs_update_buffer: true,
        };

        self.attribute_map
            .insert(attribute.label.clone(), self.attributes.len());
        self.attributes.push(attribute);
    }

    pub fn get_attribute(&self, label: &str) -> Result<&GadgetData, GeometryViewError> {
        if let Some(index) = self.attribute_map.get(label) {
            let attribute = &self.attributes[*index];
            return Ok(attribute);
        }

        Err(GeometryViewError::UnknownAttributeLabel)
    }

    pub fn get_attribute_mut(&mut self, label: &str) -> Result<&mut GadgetData, GeometryViewError> {
        if let Some(index) = self.attribute_map.get(label) {
            let attribute = &mut self.attributes[*index];
            return Ok(attribute);
        }

        Err(GeometryViewError::UnknownAttributeLabel)
    }

    pub fn set_attribute(&mut self, label: &str, data: Vec<u8>) -> Result<(), GeometryViewError> {
        let attribute = self.get_attribute_mut(label)?;

        attribute.data = data;
        attribute.needs_update_value = true;

        Ok(())
    }

    pub fn get_uniform(&self, label: &str) -> Result<&GadgetData, GeometryViewError> {
        if let Some(index) = self.uniform_map.get(label) {
            let uniform = &self.uniforms[*index];
            return Ok(uniform);
        }

        Err(GeometryViewError::UnknownUniformLabel)
    }

    pub fn get_uniform_mut(&mut self, label: &str) -> Result<&mut GadgetData, GeometryViewError> {
        if let Some(index) = self.uniform_map.get(label) {
            let uniform = &mut self.uniforms[*index];
            return Ok(uniform);
        }

        Err(GeometryViewError::UnknownUniformLabel)
    }

    pub fn add_uniform(&mut self, label: &str, index: GadgetIndex, data: Vec<u8>) {
        let uniform = GadgetData {
            label: label.to_string(),
            index,
            data,
            needs_update_value: true,
            needs_update_buffer: true,
        };

        self.uniform_map
            .insert(uniform.label.clone(), self.attributes.len());
        self.uniforms.push(uniform);
    }

    pub fn set_uniform(&mut self, label: &str, data: Vec<u8>) -> Result<(), GeometryViewError> {
        let uniform = self.get_uniform_mut(label)?;

        uniform.data = data;
        uniform.needs_update_value = true;

        Ok(())
    }

    pub fn reset_vertices(&mut self) {
        self.attributes = Vec::new();
        self.attribute_map = HashMap::new();
        self.indices = GeometryIndices::Sequential(0);
    }

    pub fn reset_uniforms(&mut self) {
        self.uniforms = Vec::new();
        self.uniform_map = HashMap::new();
    }
}

pub trait Geometry: Clone {
    fn init_view(&self, view: &mut GeometryView);
    fn update_view(&self, view: &mut GeometryView);
}

/// A minimal geometry implementation using a vector of 3D points.
///
/// This is typically used as an intermediate representation used by animations.
///
/// # Notes
/// This implementation does not modify existing indices
impl Geometry for Vec<[f32; 3]> {
    fn init_view(&self, view: &mut GeometryView) {
        view.add_attribute(
            crate::constants::POSITION_ATTR_LABEL,
            crate::constants::POSITION_ATTR_INDEX,
            Vec::<u8>::new(),
        );
    }

    fn update_view(&self, view: &mut GeometryView) {
        let mut vertices = Vec::new();

        for vertex in self {
            vertices.push(vertex[0]);
            vertices.push(vertex[1]);
            vertices.push(vertex[2]);
            vertices.push(1.0);
        }

        view.set_attribute(
            crate::constants::POSITION_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
        )
        .unwrap();
        view.get_attribute_mut(crate::constants::POSITION_ATTR_LABEL)
            .unwrap()
            .needs_update_buffer = true;
    }
}
