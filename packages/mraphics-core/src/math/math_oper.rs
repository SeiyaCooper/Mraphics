use std::ops::{Add, Mul, Sub};

pub fn lerp<T>(
    a: impl Iterator<Item = T>,
    b: impl Iterator<Item = T>,
    p: T,
) -> impl Iterator<Item = T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + 'static,
{
    a.zip(b).map(move |(x, y)| x + (y - x) * p)
}
