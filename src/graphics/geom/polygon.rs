use crate::graphics::shape::Drawing;
use super::Geom;
use std::{cmp, f32::consts::PI};
use web_sys::WebGl2RenderingContext;
pub struct RegularPolygon {
    pub radius: f32,
    pub sides: usize,
    pub color: Vec<f32>
}

impl Drawing for RegularPolygon {
    fn draw_shape(&self) -> Geom {
        let no_sides = cmp::max(3, self.sides);
        let delta = (PI * 2.0) / self.sides as f32;

        let vertices = (0..no_sides).fold(vec![], |acc, x| {
            acc.iter()
                .copied()
                .chain([
                    self.radius * (delta * x as f32).cos(),
                    self.radius * (delta * x as f32).sin(),
                ])
                .collect()
        });

        let color = self.color
            .iter()
            .cycle()
            .take(self.color.len() * no_sides)
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