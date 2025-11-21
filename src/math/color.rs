use std::ops::Deref;

use nalgebra::Vector4;

#[derive(Debug)]
pub enum ColorError {
    LengthDismatch,
    InvaildComponent,
}

pub struct Color {
    inner: Vector4<f32>,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            inner: Vector4::new(r, g, b, a),
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
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            ),
        })
    }
}

impl Deref for Color {
    type Target = [f32];
    fn deref(&self) -> &Self::Target {
        self.inner.as_slice()
    }
}
