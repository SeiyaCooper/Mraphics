use std::ops::Deref;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Color {
    inner: mraphics::Color<f64>,
}

#[wasm_bindgen]
impl Color {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            inner: mraphics::Color::new(r, g, b, a),
        }
    }

    pub fn from_hex_str(hex_str: &str) -> Self {
        let inner = mraphics::Color::from_hex_str(hex_str).unwrap();
        Self { inner }
    }
}

impl Deref for Color {
    type Target = mraphics::Color<f64>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
