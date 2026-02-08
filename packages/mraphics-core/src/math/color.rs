use std::ops::{Deref, Index};

use nalgebra::Vector4;
use num_traits::{FromPrimitive, Num, NumCast, ToPrimitive};

#[derive(Debug)]
pub enum ColorError {
    LengthDismatch,
    InvaildComponent,
}

pub trait ColorComponent: Copy + Num + NumCast + FromPrimitive + ToPrimitive {}
impl<T: Copy + Num + NumCast + FromPrimitive + ToPrimitive> ColorComponent for T {}

#[derive(Debug, Clone)]
pub struct Color<T: ColorComponent> {
    pub inner: Vector4<T>,
}

impl<T: ColorComponent> Color<T> {
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self {
            inner: Vector4::new(r, g, b, a),
        }
    }

    pub fn from_unchecked<S: ColorComponent>(color: &Color<S>) -> Self {
        Self {
            inner: Vector4::new(
                T::from(color[0]).unwrap(),
                T::from(color[1]).unwrap(),
                T::from(color[2]).unwrap(),
                T::from(color[3]).unwrap(),
            ),
        }
    }

    pub fn from_hex_str(hex_str: &str) -> Result<Self, ColorError> {
        let hex_str = hex_str.trim_start_matches('#');

        if hex_str.len() != 6 && hex_str.len() != 8 {
            return Err(ColorError::LengthDismatch);
        }

        let r = u8::from_str_radix(&hex_str[0..2], 16).map_err(|_| ColorError::InvaildComponent)?;
        let g = u8::from_str_radix(&hex_str[2..4], 16).map_err(|_| ColorError::InvaildComponent)?;
        let b = u8::from_str_radix(&hex_str[4..6], 16).map_err(|_| ColorError::InvaildComponent)?;

        let a = if hex_str.len() == 8 {
            u8::from_str_radix(&hex_str[6..8], 16).map_err(|_| ColorError::InvaildComponent)?
        } else {
            255
        };

        Ok(Color {
            inner: Vector4::new(
                T::from(r as f64 / 255.0).unwrap(),
                T::from(g as f64 / 255.0).unwrap(),
                T::from(b as f64 / 255.0).unwrap(),
                T::from(a as f64 / 255.0).unwrap(),
            ),
        })
    }

    pub fn cast_unchecked<S: ColorComponent>(color: &Color<T>) -> Color<S> {
        Color {
            inner: Vector4::new(
                S::from(color[0]).unwrap(),
                S::from(color[1]).unwrap(),
                S::from(color[2]).unwrap(),
                S::from(color[3]).unwrap(),
            ),
        }
    }
}

impl<T: ColorComponent> Deref for Color<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.inner.as_slice()
    }
}

impl<T: ColorComponent> Index<usize> for Color<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T: ColorComponent> Into<Color<T>> for [T; 4] {
    fn into(self) -> Color<T> {
        Color::new(self[0], self[1], self[2], self[3])
    }
}
