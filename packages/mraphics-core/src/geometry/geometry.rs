use crate::{GadgetData, GadgetIndex, InstanceUpdater, Interpolatable, constants};
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

/// A view of geometric data that can be consumed by shaders.
///
/// This struct represents a collection of attributes, uniforms, and indices
/// that define a geometry.
#[derive(Debug)]
pub struct GeometryView {
    /// Vertex attributes of the geometry (e.g., position, normal, color).
    pub attributes: Vec<GadgetData>,

    /// Maps attribute labels to their indices in [`Self::attributes`].
    /// Used for querying a attribute by its label.
    attribute_map: HashMap<String, usize>,

    /// Uniform variables of the geometry.
    pub uniforms: Vec<GadgetData>,

    /// Maps uniform labels to their indices in [`Self::uniforms`].
    /// Used for querying a uniform by its label.
    uniform_map: HashMap<String, usize>,

    /// Index buffer specifying how vertices are connected.
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

/// A trait for objects that can both initialize and update a [`GeometryView`].
///
/// Types implementing this trait can:
/// - Create a complete geometry view from scratch
/// - Modify an existing geometry view
/// - Are cloneable to support duplication of geometric data.
///
/// # Required Trait Bounds
/// - [`Clone`]: For copying geometric data
pub trait Geometry: Clone {
    /// Initializes a new [`GeometryView`].
    fn init_view(&self, view: &mut GeometryView);

    /// Updates an existing [`GeometryView`] with this object's data.
    fn update_view(&self, view: &mut GeometryView);

    /// Initializes self before initializing geometry view, optional.
    fn init(&mut self) {}
}

/// A collection of vertices in homogeneous coordinates (x, y, z, w).
///
/// This is typically used as an intermediate representation, especially in animations.
#[derive(Clone)]
pub struct Vertices {
    pub data: Vec<[f32; 4]>,
}

impl Vertices {
    /// Craetes a new collection of vertices.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Applies a transform.
    pub fn apply_transform<Trans: Fn(&[f32; 4]) -> [f32; 4]>(&self, transform: Trans) -> Self {
        Self {
            data: self.data.iter().map(transform).collect(),
        }
    }

    /// Updates a instance with [`Vertices`].
    ///
    /// # Notes
    /// This implementation does not modify existing indices
    pub fn update_geometry_view(&self, view: &mut GeometryView) {
        let mut vertices = Vec::new();

        for vertex in &self.data {
            vertices.push(vertex[0]);
            vertices.push(vertex[1]);
            vertices.push(vertex[2]);
            vertices.push(vertex[3]);
        }

        view.set_attribute(
            crate::constants::POSITION_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
        )
        .unwrap();
    }
}

impl Interpolatable for Vertices {
    fn interpolate(&self, to: &Self, p: f32) -> Self {
        Self {
            data: self.data.interpolate(&to.data, p),
        }
    }
}

/// Updates a instance with [`Vertices`].
///
/// # Notes
/// This implementation only modifies geometry view and it does not modify existing indices
impl InstanceUpdater for Vertices {
    fn update_instance(&self, instance: &mut super::RenderInstance) {
        self.update_geometry_view(&mut instance.geometry);
    }
}
