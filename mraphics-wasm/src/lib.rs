#![cfg(target_arch = "wasm32")]

mod canvas;
pub use canvas::*;

mod math;
pub use math::*;

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen(start)]
fn set_up() {
    console_error_panic_hook::set_once();

    #[cfg(debug_assertions)]
    {
        use console_log;
        console_log::init().unwrap();
    }
}
