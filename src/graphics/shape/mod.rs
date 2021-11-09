pub mod quad;
pub mod triangle;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Shape {
    vertices: Vec<f32>,
    color: Vec<f32>,
    pub mode: u32,
    pub vertex_count: i32,
}

impl Shape {
    pub fn position(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    pub fn color(&self) -> Vec<f32> {
        self.color.clone()
    }
}
