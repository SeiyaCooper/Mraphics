use std::f32::consts::PI;

use crate::math::{Matrix, Vector};

pub trait Camera {
    fn view_mat_data(&self) -> &[u8];
    fn projection_mat_data(&self) -> &[u8];

    fn view_mat(&self) -> &Matrix<f32>;
    fn projection_mat(&self) -> &Matrix<f32>;
}

pub struct PerspectiveCamera {
    view_mat: Matrix<f32>,
    pub view_mat_data: Vec<u8>,

    projection_mat: Matrix<f32>,
    pub projection_mat_data: Vec<u8>,

    pub center: Vector<f32>,
    pub rotation: Vector<f32>,

    pub far: f32,
    pub near: f32,
    pub aspect: f32,
    pub fov_rad: f32,
}

impl PerspectiveCamera {
    pub fn update_data(&mut self) {
        self.view_mat_data = self.view_mat.as_static::<4, 4>().as_bytes().to_vec();
        self.projection_mat_data = self.projection_mat.as_static::<4, 4>().as_bytes().to_vec();
    }

    pub fn update_matrix(&mut self) {
        self.view_mat = PerspectiveCamera::compute_view_mat(&self.center, &self.rotation);
        self.projection_mat = PerspectiveCamera::compute_projection_mat(
            self.far,
            self.near,
            self.aspect,
            self.fov_rad,
        )
    }

    pub fn compute_projection_mat(far: f32, near: f32, aspect: f32, fov_rad: f32) -> Matrix<f32> {
        let f = far;
        let n = near;
        let a = aspect;
        let c = (fov_rad / 2.0).tan();

        Matrix::new(vec![
            Vector::new(vec![1.0 / (a * c), 0.0, 0.0, 0.0]),
            Vector::new(vec![0.0, 1.0 / c, 0.0, 0.0]),
            Vector::new(vec![0.0, 0.0, (f + n) / (n - f), -1.0]),
            Vector::new(vec![0.0, 0.0, (2.0 * f * n) / (n - f), 0.0]),
        ])
    }

    pub fn compute_view_mat(center: &Vector<f32>, rotation: &Vector<f32>) -> Matrix<f32> {
        Matrix::rotate_x(-rotation[0], 4)
            * Matrix::rotate_y(-rotation[1], 4)
            * Matrix::rotate_z(-rotation[2], 4)
            * Matrix::translate(-center[0], -center[1], -center[2])
    }
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        let (far, near, aspect, fov_rad) = (1000.0, 0.1, 1.0, PI / 4.0);
        let (center, rotation) = (
            Vector::new(vec![0.0, 0.0, 5.0]),
            Vector::new(vec![0.0, 0.0, 0.0]),
        );
        let projection_mat = PerspectiveCamera::compute_projection_mat(far, near, aspect, fov_rad);
        let view_mat = PerspectiveCamera::compute_view_mat(&center, &rotation);
        Self {
            view_mat_data: view_mat.as_static::<4, 4>().as_bytes().to_vec(),
            view_mat,
            projection_mat_data: projection_mat.as_static::<4, 4>().as_bytes().to_vec(),
            projection_mat,
            center,
            rotation,
            far,
            near,
            aspect,
            fov_rad,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn view_mat(&self) -> &Matrix<f32> {
        &self.view_mat
    }

    fn projection_mat(&self) -> &Matrix<f32> {
        &self.projection_mat
    }

    fn view_mat_data(&self) -> &[u8] {
        &self.view_mat_data
    }

    fn projection_mat_data(&self) -> &[u8] {
        &self.projection_mat_data
    }
}
