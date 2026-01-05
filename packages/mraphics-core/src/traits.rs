//! Shared traits that are used across multiple modules.

pub trait Interpolatable {
    fn interpolate(&self, to: &Self, p: f32) -> Self;
}

impl Interpolatable for f32 {
    fn interpolate(&self, to: &Self, p: f32) -> Self {
        self + (to - self) * p
    }
}

impl Interpolatable for f64 {
    fn interpolate(&self, to: &Self, p: f32) -> Self {
        self + (to - self) * (p as f64)
    }
}

impl<E: Interpolatable, const N: usize> Interpolatable for [E; N] {
    fn interpolate(&self, to: &Self, p: f32) -> Self {
        // SAFETY: The closure passed to `array::from_fn` is only called for indices in the range `0..N`.
        // Since `N` is the array length, both `self[i]` and `to[i]` are guaranteed to be valid indices.

        std::array::from_fn(|i| unsafe {
            self.get_unchecked(i).interpolate(to.get_unchecked(i), p)
        })
    }
}

impl<E: Interpolatable> Interpolatable for Vec<E> {
    fn interpolate(&self, to: &Self, p: f32) -> Self {
        self.iter()
            .zip(to.iter())
            .map(|(from, to)| from.interpolate(to, p))
            .collect()
    }
}
