pub trait Scalar: Copy + Clone + Default {}

impl<T: Copy + Clone + Default> Scalar for T {}
