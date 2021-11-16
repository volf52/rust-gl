use crate::graphics::shape::{Drawing, calc_n_vertices, color_n_vertices};
use super::Geom;
use web_sys::WebGl2RenderingContext;
pub struct Ellipse {
    pub width: f32,
    pub height: f32,
    pub color: Vec<f32>,
}
pub struct Circle {
    pub radius: f32,
    pub color: Vec<f32>
}

impl Drawing for Ellipse {
    fn draw_shape(&self) -> Geom {
        let vertex_count: i32 = 200;

        let vertices = calc_n_vertices(&self.width, &self.height, vertex_count as u32);
        let color = color_n_vertices(&self.color, vertex_count as usize);

        Geom {
            vertices,
            color,
            vertex_count,
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
        }
    }
}

impl Drawing for Circle {
    fn draw_shape(&self) -> Geom {
        Ellipse {
            width: self.radius,
            height: self.radius,
            color: self.color.clone(),
        }
        .draw_shape()
    }
}

