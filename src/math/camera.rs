use std::f32::consts::PI;

use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};

pub trait Camera {
    fn view_mat_data(&self) -> &[u8];
    fn projection_mat_data(&self) -> &[u8];
}

pub struct PerspectiveCamera {
    pub view_transform: Isometry3<f32>,
    view_mat: Matrix4<f32>,

    pub up: Vector3<f32>,
    center: Vector3<f32>,

    pub proj_transform: Perspective3<f32>,
    proj_mat: Matrix4<f32>,
}

impl PerspectiveCamera {
    pub fn center(&self) -> &Vector3<f32> {
        &self.center
    }

    pub fn set_center(&mut self, center: &Vector3<f32>) {
        self.center.copy_from(&-center);
        self.view_transform.translation.vector.copy_from(&-center);
        self.view_mat = self.view_transform.to_homogeneous();
    }

    pub fn set_rotation(&mut self, rotarion: &Vector3<f32>) {
        self.view_transform =
            Isometry3::new(self.view_transform.translation.vector, rotarion.clone());
        self.view_mat = self.view_transform.to_homogeneous();
    }

    pub fn look_at(&mut self, target: &Point3<f32>) {
        self.view_transform = Isometry3::look_at_rh(
            &Point3::from_slice(&self.center.as_slice()),
            target,
            &self.up,
        );
        self.view_mat = self.view_transform.to_homogeneous();
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.proj_transform.set_aspect(aspect);
        self.proj_mat = self.proj_transform.to_homogeneous();
    }

    pub fn set_far(&mut self, far: f32) {
        self.proj_transform.set_zfar(far);
        self.proj_mat = self.proj_transform.to_homogeneous();
    }

    pub fn set_near(&mut self, near: f32) {
        self.proj_transform.set_znear(near);
        self.proj_mat = self.proj_transform.to_homogeneous();
    }

    pub fn set_fov_rad(&mut self, fov_rad: f32) {
        self.proj_transform.set_zfar(fov_rad);
        self.proj_mat = self.proj_transform.to_homogeneous();
    }
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        let (far, near, aspect, fov_rad) = (1000.0, 0.1, 1.0, PI / 4.0);
        let (center, rotation) = (Vector3::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 0.0));
        let (view_transform, proj_transform) = (
            Isometry3::new(-center, rotation),
            Perspective3::new(aspect, fov_rad, near, far),
        );

        Self {
            view_mat: view_transform.to_homogeneous(),
            view_transform,
            proj_mat: proj_transform.to_homogeneous(),
            proj_transform,

            up: Vector3::y(),
            center,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn view_mat_data(&self) -> &[u8] {
        bytemuck::cast_slice(self.view_mat.as_slice())
    }

    fn projection_mat_data(&self) -> &[u8] {
        bytemuck::cast_slice(self.proj_mat.as_slice())
    }
}
