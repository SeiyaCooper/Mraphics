use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Scalar:
    Copy
    + Clone
    + Default
    + bytemuck::Pod
    + bytemuck::Zeroable
    + Sized
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
{
    const ONE: Self;
}

macro_rules! impl_scalar {
    ($t: ty, $one:expr) => {
        impl Scalar for $t {
            const ONE: Self = $one;
        }
    };
}

impl_scalar!(i8, 1);
impl_scalar!(i16, 1);
impl_scalar!(i32, 1);
impl_scalar!(i64, 1);
impl_scalar!(i128, 1);
impl_scalar!(isize, 1);

impl_scalar!(f32, 1.0);
impl_scalar!(f64, 1.0);
