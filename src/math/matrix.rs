use std::ops::{Index, IndexMut};

use crate::math::{Scalar, Vector};

#[derive(Debug, Clone)]
pub struct Matrix<T: Scalar> {
    pub data: Vec<Vector<T>>,
}

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
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

    pub fn identity(n: usize) -> Self
    where
        T: From<u8>,
    {
        let mut out: Matrix<T> = Matrix::zeros(n, n);

        for i in 0..n {
            out.data[i][i] = T::from(1);
        }

        out
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
