use std::f32::consts::PI;

use super::Geom;

use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
impl Geom {
    pub fn new_ellipse(width: f32, height: f32, color_unit: Vec<f32>) -> Self {
        let vertex_count: i32 = 400;
        let k = 2.0 * PI / 180.0 as f32;

        let vertices = (1..vertex_count / 2).fold(vec![], |acc, x| {
            acc.iter()
                .copied()
                .chain([
                    width * (k * x as f32).cos(),
                    height * (k * x as f32).sin(),
                    0.0,
                    0.0
                ])
                .collect()
        });

        let color = color_unit
            .iter()
            .cycle()
            .take(color_unit.len() * vertex_count as usize)
            .map(|f| f.clone())
            .collect();

        Geom {
            vertices,
            color,
            vertex_count,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
        }
    }
}
