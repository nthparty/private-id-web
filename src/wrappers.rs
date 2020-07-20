use super::alib;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(a: &str, b: &str) -> String {
    // format!("Hello, {}!", a)
    alib::arithmetic::add(a.len(), b.len()).to_string()
}
