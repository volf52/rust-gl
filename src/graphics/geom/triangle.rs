use super::Geom;

use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
impl Geom {
    pub fn new_triangle(size: f32, color_unit: Vec<f32>) -> Self {
        let right = size / 2.0;
        let left = -right;
        let top = size / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom].to_vec();
        
        let color = color_unit
            .iter()
            .cycle()
            .take(color_unit.len() * 3)
            .map(|f| f.clone())
            .collect();

        Geom {
            vertices,
            color,
            vertex_count: 3,
            mode: WebGl2RenderingContext::TRIANGLES,
        }
    }
}
