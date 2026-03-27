use crate::{
    CustomIndices, Geometry, GeometryIndices, Material, Mesh, Representable, Transformable,
    Vertices,
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

    pub vertices: Vertices,
    pub indices: Vec<u16>,
}

impl Sphere {
    pub fn new() -> Self {
        Self::default()
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
            vertices: Vertices::new(),
            indices: Vec::new(),
        }
    }
}

impl Geometry for Sphere {
    fn init_view(&self, view: &mut super::GeometryView) {
        view.add_storage(
            crate::constant::POSITION_STORAGE_LABEL,
            crate::constant::POSITION_STORAGE_INDEX,
            Vec::<u8>::new(),
        );
    }

    fn update_view(&self, view: &mut super::GeometryView) {
        self.vertices.update_geometry_view(view);

        view.get_storage_mut(crate::constant::POSITION_STORAGE_LABEL)
            .unwrap()
            .needs_update_buffer = true;
        view.indices = GeometryIndices::CustomU16(CustomIndices::new((&self.indices).to_owned()));
    }

    fn update(&mut self) {
        self.vertices = Vertices::new();
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

                self.vertices.data.push([
                    r * phi.cos() * theta.sin(),
                    r * theta.cos(),
                    r * phi.sin() * theta.sin(),
                    1.0,
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

impl<M: Material> Representable for Mesh<Sphere, M> {
    type Intermediate = Vertices;

    fn as_intermediate(&self) -> Self::Intermediate {
        self.geometry.vertices.clone()
    }

    fn update_from_intermediate(&mut self, repr: &Self::Intermediate) {
        self.geometry.vertices = repr.clone();
    }
}

impl<M: Material> Transformable for Mesh<Sphere, M> {
    fn apply_transform<Trans: Fn(&[f32; 3]) -> [f32; 3]>(
        &self,
        transform: Trans,
    ) -> Self::Intermediate {
        self.geometry.vertices.apply_transform(|vertex| {
            let transformed = transform(&[vertex[0], vertex[1], vertex[2]]);
            [transformed[0], transformed[1], transformed[2], 1.0]
        })
    }
}
