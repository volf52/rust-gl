use super::Shape;

use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
impl Shape {
    pub fn new_triangle(size: f32) -> Shape {
        let right = size / 2.0;
        let left = -right;
        let top = size / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom].to_vec();
        let color = [
            0.0, 0.0, 0.0, // vertex 1
            0.0, 0.0, 0.0, // vertex 2
            1.0, 0.0, 0.0, // vertex 3
        ]
        .to_vec();

        Shape {
            vertices,
            color,
            vertex_count: 3,
            mode: WebGl2RenderingContext::TRIANGLES,
        }
    }
}
