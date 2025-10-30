use std::{
    ops::{Index, IndexMut, Mul},
    vec,
};

use crate::math::{Scalar, Vector};

#[derive(Debug, Clone)]
pub struct Matrix<T: Scalar> {
    pub data: Vec<Vector<T>>,
}

impl<T: Scalar> Matrix<T> {
    pub fn new(data: Vec<Vector<T>>) -> Self {
        Self { data }
    }

    pub fn as_static<const ROW: usize, const COL: usize>(&self) -> SMatrix<T, ROW, COL> {
        let mut out = SMatrix::<T, ROW, COL>::new();

        for i in 0..COL {
            for j in 0..ROW {
                out[i][j] = self[(i, j)];
            }
        }

        out
    }

    pub fn zeros(row_num: usize, col_num: usize) -> Self {
        Self {
            data: vec![Vector::zeros(row_num); col_num],
        }
    }

    pub fn identity(n: usize) -> Self {
        let mut out: Matrix<T> = Matrix::zeros(n, n);

        for i in 0..n {
            out.data[i][i] = T::ONE;
        }

        out
    }

    pub fn col_num(&self) -> usize {
        self.data.len()
    }

    pub fn row_num(&self) -> usize {
        self.data[0].len()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.col_num(), self.row_num())
    }

    pub fn rotate_x(ang_rad: f32, n: usize) -> Self
    where
        T: From<f32>,
    {
        let one = T::ONE;
        let zero = T::default();
        let sin = T::from(ang_rad.sin());
        let cos = T::from(ang_rad.cos());

        if n == 3 {
            return Matrix {
                data: vec![
                    Vector::new(vec![one, zero, zero]),
                    Vector::new(vec![zero, cos, -sin]),
                    Vector::new(vec![zero, sin, cos]),
                ],
            };
        } else if n == 4 {
            return Matrix {
                data: vec![
                    Vector::new(vec![one, zero, zero, zero]),
                    Vector::new(vec![zero, cos, -sin, zero]),
                    Vector::new(vec![zero, sin, cos, zero]),
                    Vector::new(vec![zero, zero, zero, one]),
                ],
            };
        } else {
            panic!(
                "Invaild Dimension: Matrix::rotate_x only supports generating 3x3 or 4x4 matrices, got {}",
                n
            )
        }
    }

    pub fn rotate_y(ang_rad: f32, n: usize) -> Self
    where
        T: From<f32>,
    {
        let one = T::ONE;
        let zero = T::default();
        let sin = T::from(ang_rad.sin());
        let cos = T::from(ang_rad.cos());

        if n == 3 {
            return Matrix {
                data: vec![
                    Vector::new(vec![cos, zero, -sin]),
                    Vector::new(vec![zero, one, zero]),
                    Vector::new(vec![sin, zero, cos]),
                ],
            };
        } else if n == 4 {
            return Matrix {
                data: vec![
                    Vector::new(vec![cos, zero, -sin, zero]),
                    Vector::new(vec![zero, one, zero, zero]),
                    Vector::new(vec![sin, zero, cos, zero]),
                    Vector::new(vec![zero, zero, zero, one]),
                ],
            };
        } else {
            panic!(
                "Invaild Dimension: Matrix::rotate_y only supports generating 3x3 or 4x4 matrices, got {}",
                n
            )
        }
    }

    pub fn rotate_z(ang_rad: f32, n: usize) -> Self
    where
        T: From<f32>,
    {
        let one = T::ONE;
        let zero = T::default();
        let sin = T::from(ang_rad.sin());
        let cos = T::from(ang_rad.cos());

        if n == 3 {
            return Matrix {
                data: vec![
                    Vector::new(vec![cos, -sin, zero]),
                    Vector::new(vec![sin, cos, zero]),
                    Vector::new(vec![zero, zero, one]),
                ],
            };
        } else if n == 4 {
            return Matrix {
                data: vec![
                    Vector::new(vec![cos, -sin, zero, zero]),
                    Vector::new(vec![sin, cos, zero, zero]),
                    Vector::new(vec![zero, zero, one, zero]),
                    Vector::new(vec![zero, zero, zero, one]),
                ],
            };
        } else {
            panic!(
                "Invaild Dimension: Matrix::rotate_z only supports generating 3x3 or 4x4 matrices, got {}",
                n
            )
        }
    }
}

impl<T: Scalar> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T: Scalar> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<T: Scalar> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut out = Matrix::zeros(self.row_num(), self.col_num());

        for i in 0..self.col_num() {
            for j in 0..self.row_num() {
                out[(i, j)] = self[(i, j)] * rhs;
            }
        }

        out
    }
}

/**
 * Static Matrix
 */
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SMatrix<T: Scalar, const ROW: usize, const COL: usize> {
    pub data: [[T; ROW]; COL],
}

impl<T: Scalar, const ROW: usize, const COL: usize> SMatrix<T, ROW, COL> {
    pub fn new() -> Self {
        Self {
            data: [[T::default(); ROW]; COL],
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.data)
    }
}

impl<T: Scalar, const ROW: usize, const COL: usize> Index<usize> for SMatrix<T, ROW, COL> {
    type Output = [T; ROW];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Scalar, const ROW: usize, const COL: usize> IndexMut<usize> for SMatrix<T, ROW, COL> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
