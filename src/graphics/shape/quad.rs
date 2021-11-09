use super::Shape;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
impl Shape {
    pub fn new_rectangle(width: f32, height: f32) -> Self {
        let right = width / 2.0;
        let left = -right;
        let top = height / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom, right, bottom].to_vec();
        let color = [
            1.0, 0.4, 0.4, // vertex 1
            1.0, 0.0, 0.0, // vertex 2
            0.0, 1.0, 0.0, // vertex 3
            0.0, 0.0, 1.0, // vertex 4
        ]
        .to_vec();

        Shape {
            vertices,
            color,
            vertex_count: 4,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
        }
    }

    pub fn new_square(size: f32) -> Self {
        Shape::new_rectangle(size, size)
    }
}
