use crate::math::Matrix;

pub struct Camera {
    pub view_mat: Matrix<f32>,
    pub projection_mat: Matrix<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            view_mat: Matrix::identity(4),
            projection_mat: Matrix::identity(4),
        }
    }
}
