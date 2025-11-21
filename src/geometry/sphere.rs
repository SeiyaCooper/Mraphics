use crate::{geometry::Geometry, impl_inner_geometry_view};
use std::f32::consts::PI;

pub struct SphereDescriptor {
    pub radius: f32,

    pub phi_start: f32,
    pub phi_end: f32,
    pub phi_segments: u32,

    pub theta_start: f32,
    pub theta_end: f32,
    pub theta_segments: u32,
}

impl Default for SphereDescriptor {
    fn default() -> Self {
        Self {
            radius: 1.0,
            phi_start: 0.0,
            phi_end: PI * 2.0,
            phi_segments: 32,
            theta_start: 0.0,
            theta_end: PI,
            theta_segments: 16,
        }
    }
}

const SPHERE_IDENTIFIER_PREFIX: &'static str = "mraphics-sphere-";

pub struct Sphere {
    pub inner: Geometry,
}

impl Sphere {
    pub fn new(desc: &SphereDescriptor) -> Self {
        let out = Self {
            inner: Geometry::with_id_prefix(SPHERE_IDENTIFIER_PREFIX.to_string()),
        };

        out
    }
}

impl_inner_geometry_view!(Sphere);
