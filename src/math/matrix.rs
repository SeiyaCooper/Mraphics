use crate::math::{Scalar, Vector};

#[derive(Debug, Clone)]
pub struct Matrix<T>
where
    T: Scalar,
{
    pub data: Vec<Vector<T>>,
}

impl<T> Matrix<T>
where
    T: Scalar,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
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
