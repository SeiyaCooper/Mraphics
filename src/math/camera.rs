use crate::math::{Matrix, Vector};

pub trait Camera {
    fn view_mat_data(&self) -> &[u8];
    fn projection_mat_data(&self) -> &[u8];

    fn view_mat(&self) -> &Matrix<f32>;
    fn projection_mat(&self) -> &Matrix<f32>;
}

pub struct PerspectiveCamera {
    pub view_mat: Matrix<f32>,
    pub view_mat_data: Vec<u8>,

    pub projection_mat: Matrix<f32>,
    pub projection_mat_data: Vec<u8>,

    target: Vector<f32>,
    rotation: Vector<f32>,
    up_dir: Vector<f32>,

    far: f32,
    near: f32,
    aspect: f32,
    fov: f32,
}

impl PerspectiveCamera {}

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
