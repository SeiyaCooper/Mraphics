use crate::{
    CustomIndices, GadgetData, Geometry, GeometryIndices, Material, Mesh, Transformable,
    math_oper::lerp,
};
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Sphere {
    pub radius: f32,

    pub phi_start: f32,
    pub phi_end: f32,
    pub phi_segments: u16,

    pub theta_start: f32,
    pub theta_end: f32,
    pub theta_segments: u16,

    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u16>,
}

impl Sphere {
    pub fn new() -> Self {
        let mut out = Self::default();
        out.update();
        out
    }

    pub fn update(&mut self) {
        self.vertices = Vec::new();
        self.indices = Vec::new();

        let r = self.radius;
        let phi_unit = (self.phi_end - self.phi_start) / self.phi_segments as f32;
        let theta_unit = (self.theta_end - self.theta_start) / self.theta_segments as f32;

        for i in 0..=self.theta_segments {
            let i = i as f32;
            for j in 0..self.phi_segments {
                let j = j as f32;

                let phi = self.phi_start + j * phi_unit;
                let theta = self.theta_start + i * theta_unit;

                self.vertices.push([
                    r * phi.cos() * theta.sin(),
                    r * theta.cos(),
                    r * phi.sin() * theta.sin(),
                ]);
            }
        }

        let mut add_plane = |a: u16, b: u16, c: u16, d: u16| {
            self.indices.push(a);
            self.indices.push(b);
            self.indices.push(d);

            self.indices.push(b);
            self.indices.push(c);
            self.indices.push(d);
        };

        for i in 0..self.theta_segments {
            for j in 0..self.phi_segments {
                let next = if j + 1 == self.phi_segments { 0 } else { j + 1 };

                if (self.phi_start > 0.0 || self.phi_end < PI * 2.0) && next == 0 {
                    continue;
                }

                let offset = i * self.phi_segments;
                let a = offset + j;
                let b = offset + self.phi_segments + j;
                let c = offset + self.phi_segments + next;
                let d = offset + next;

                add_plane(a, b, c, d);
            }
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            radius: 1.0,
            phi_start: 0.0,
            phi_end: PI * 2.0,
            phi_segments: 32,
            theta_start: 0.0,
            theta_end: PI,
            theta_segments: 16,
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}

impl Geometry for Sphere {
    fn update_view(&self, view: &mut super::GeometryView) {
        let mut vertices = Vec::new();

        for vertex in &self.vertices {
            vertices.push(vertex[0]);
            vertices.push(vertex[1]);
            vertices.push(vertex[2]);
            vertices.push(1.0);
        }

        view.reset_vertices();

        view.attributes.push(GadgetData {
            label: String::from(crate::constants::POSITION_ATTR_LABEL),
            index: crate::constants::POSITION_ATTR_INDEX,
            data: Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
            needs_update_value: true,
            needs_update_buffer: true,
        });
        view.indices = GeometryIndices::CustomU16(CustomIndices::new((&self.indices).to_owned()));
    }
}

impl<M: Material> Transformable for Mesh<Sphere, M> {
    fn apply_transform<Trans: Fn(&[f32; 3]) -> [f32; 3]>(
        &self,
        transform: Trans,
        progress: f32,
    ) -> Self {
        let mut transformed_geometry = Sphere::default();

        for vertex in &self.geometry.vertices {
            let mut iter = lerp(
                vertex.iter().copied(),
                transform(vertex).into_iter(),
                progress,
            );

            transformed_geometry.vertices.push([
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]);
        }

        transformed_geometry.indices = self.geometry.indices.to_owned();

        Self {
            identifier: self.identifier,
            geometry: transformed_geometry,
            material: self.material.clone(),
        }
    }
}
