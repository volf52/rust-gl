use super::Geom;
use std::{cmp, f32::consts::PI};
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
use crate::hey;
#[wasm_bindgen]
impl Geom {
    pub fn new_polygon(radius: f32, sides: usize, color_unit: Vec<f32>) -> Self {
        let no_sides = sides;
        let delta = (PI * 2.0) / sides as f32;

        let vertices = (0..no_sides).fold(vec![], |acc, x| {
            acc.iter()
                .copied()
                .chain([
                    radius * (delta * x as f32).cos(),
                    radius * (delta * x as f32).sin(),
                ])
                .collect()
        });

        hey(vertices.clone());
        let color = color_unit
            .iter()
            .cycle()
            .take(color_unit.len() * no_sides)
            .map(|f| f.clone())
            .collect();

        Geom {
            vertices,
            color,
            vertex_count: no_sides as i32,
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
        }
    }
}
