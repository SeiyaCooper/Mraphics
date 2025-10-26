use crate::math::Scalar;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Vector<T>
where
    T: Scalar,
{
    pub data: Vec<T>,
}

impl<T> Vector<T>
where
    T: Scalar,
{
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn zeros(n: usize) -> Self {
        Self {
            data: vec![Default::default(); n],
        }
    }
}

impl<T> Index<usize> for Vector<T>
where
    T: Scalar,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector<T>
where
    T: Scalar,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
