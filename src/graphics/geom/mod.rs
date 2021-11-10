mod geom;
mod circle;
pub mod quad;
pub mod triangle;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Geom {
    vertices: Vec<f32>,
    color: Vec<f32>,
    pub mode: u32,
    pub vertex_count: i32,
}
