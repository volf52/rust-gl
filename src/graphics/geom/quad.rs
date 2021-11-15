use super::Geom;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
impl Geom {
    pub fn new_rectangle(width: f32, height: f32, color_unit: Vec<f32>) -> Self {
        let right = width / 2.0;
        let left = -right;
        let top = height / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom, right, bottom].to_vec();
        let color = color_unit
            .iter()
            .cycle()
            .take(color_unit.len() * 4)
            .map(|f| f.clone())
            .collect();

        Geom {
            vertices,
            color,
            vertex_count: 4,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
        }
    }
}

