use crate::{
    geometry::{CustomIndices, Geometry, GeometryIndices, GeometryView},
    impl_inner_geometry_view,
    render::GadgetData,
};
use std::f32::consts::PI;

pub struct SphereDescriptor {
    pub radius: f32,

    pub phi_start: f32,
    pub phi_end: f32,
    pub phi_segments: u16,

    pub theta_start: f32,
    pub theta_end: f32,
    pub theta_segments: u16,
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
        let mut out = Self {
            inner: Geometry::with_id_prefix(SPHERE_IDENTIFIER_PREFIX.to_string()),
        };

        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let r = desc.radius;
        let phi_unit = (desc.phi_end - desc.phi_start) / desc.phi_segments as f32;
        let theta_unit = (desc.theta_end - desc.theta_start) / desc.theta_segments as f32;

        for i in 0..desc.theta_segments {
            let i = i as f32;
            for j in 0..desc.phi_segments {
                let j = j as f32;

                let phi = desc.phi_start + j * phi_unit;
                let theta = desc.theta_start + i * theta_unit;

                vertices.push(r * phi.cos() * theta.sin());
                vertices.push(r * theta.cos());
                vertices.push(r * phi.sin() * theta.sin());
                vertices.push(1.0);
            }
        }

        let mut add_plane = |a: u16, b: u16, c: u16, d: u16| {
            indices.push(a);
            indices.push(b);
            indices.push(d);

            indices.push(b);
            indices.push(c);
            indices.push(d);
        };

        for i in 0..desc.theta_segments {
            for j in 0..desc.phi_segments {
                let next = if j + 1 == desc.phi_segments { 0 } else { j + 1 };

                if (desc.phi_start > 0.0 || desc.phi_end < PI * 2.0) && next == 0 {
                    continue;
                }

                let offset = i * desc.phi_segments;
                let a = offset + j;
                let b = offset + desc.phi_segments + j;
                let c = offset + desc.phi_segments + next;
                let d = offset + next;

                add_plane(a, b, c, d);
            }
        }

        out.attributes_mut().push(GadgetData {
            label: String::from(crate::constants::POSITION_ATTR_LABEL),
            index: crate::constants::POSITION_ATTR_INDEX,
            data: Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
            needs_update_value: true,
            needs_update_buffer: true,
        });
        out.inner.indices = GeometryIndices::CustomU16(CustomIndices::new(indices));

        out
    }
}

impl_inner_geometry_view!(Sphere);
