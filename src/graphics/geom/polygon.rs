use crate::graphics::shape::{Drawing, calc_n_vertices, color_n_vertices};
use super::Geom;
use std::cmp;
use web_sys::WebGl2RenderingContext;
pub struct IrregularPolygon {
    pub width: f32,
    pub height: f32,
    pub sides: usize,
    pub color: Vec<f32>
}
pub struct RegularPolygon {
    pub radius: f32,
    pub sides: usize,
    pub color: Vec<f32>
}

impl Drawing for IrregularPolygon {
    fn draw_shape(&self) -> Geom {
        let no_sides = cmp::max(3, self.sides);

        let vertices = calc_n_vertices(&self.width, &self.height, no_sides as u32);
        let color = color_n_vertices(&self.color, no_sides.clone() as usize);

        Geom {
            vertices,
            color,
            vertex_count: no_sides as i32,
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
        }
    }
}

impl Drawing for RegularPolygon {
    fn draw_shape(&self) -> Geom {
        IrregularPolygon {
            width: self.radius,
            height: self.radius,
            sides: self.sides,
            color: self.color.clone(),
        }
        .draw_shape()
    }
}