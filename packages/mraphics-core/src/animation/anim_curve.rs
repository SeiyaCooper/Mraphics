use std::f32::consts::PI;

pub trait AnimCurve {
    fn sample(&self, p: f32) -> f32;
}

pub struct Linear;
impl AnimCurve for Linear {
    fn sample(&self, p: f32) -> f32 {
        p
    }
}

pub struct EaseInQuad;
impl AnimCurve for EaseInQuad {
    fn sample(&self, p: f32) -> f32 {
        p * p
    }
}

pub struct EaseOutQuad;
impl AnimCurve for EaseOutQuad {
    fn sample(&self, p: f32) -> f32 {
        1.0 - (1.0 - p) * (1.0 - p)
    }
}

pub struct EaseInOutQuad;
impl AnimCurve for EaseInOutQuad {
    fn sample(&self, p: f32) -> f32 {
        if p < 0.5 {
            2.0 * p * p
        } else {
            1.0 - 2.0 * (1.0 - p) * (1.0 - p)
        }
    }
}

pub struct EaseInCubic;
impl AnimCurve for EaseInCubic {
    fn sample(&self, p: f32) -> f32 {
        p * p * p
    }
}

pub struct EaseOutCubic;
impl AnimCurve for EaseOutCubic {
    fn sample(&self, p: f32) -> f32 {
        1.0 - (1.0 - p) * (1.0 - p) * (1.0 - p)
    }
}

pub struct EaseInOutCubic;
impl AnimCurve for EaseInOutCubic {
    fn sample(&self, p: f32) -> f32 {
        if p < 0.5 {
            4.0 * p * p * p
        } else {
            1.0 - 4.0 * (1.0 - p) * (1.0 - p) * (1.0 - p)
        }
    }
}

pub struct EaseInSine;
impl AnimCurve for EaseInSine {
    fn sample(&self, p: f32) -> f32 {
        1.0 - (p * PI / 2.0).cos()
    }
}

pub struct EaseOutSine;
impl AnimCurve for EaseOutSine {
    fn sample(&self, p: f32) -> f32 {
        (p * PI / 2.0).sin()
    }
}

pub struct EaseInOutSine;
impl AnimCurve for EaseInOutSine {
    fn sample(&self, p: f32) -> f32 {
        -((PI * p).cos() - 1.0) / 2.0
    }
}
