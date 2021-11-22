use crate::graphics::shape::{calc_n_vertices, color_n_vertices};
use web_sys::WebGl2RenderingContext;
use crate::math::Matrix;
use super::geom::Geom;
pub struct Ellipse {
    pub width: f32,
    pub height: f32,
    pub color: Vec<f32>
}
pub struct Circle {
    pub radius: f32,
    pub color: Vec<f32>
}

impl Ellipse {
    fn new(width: f32, height: f32, color: Vec<f32>) -> Geom {
        let vertex_count: i32 = 200;

        let vertices = calc_n_vertices(&width, &height, vertex_count as u32);
        let color = color_n_vertices(&color, vertex_count as usize);
        let matrix = Matrix::new();
        
        Geom {
            vertices,
            color,
            vertex_count,
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            matrix
        }
    }
}

impl Circle {
    fn draw_shape(radius: f32, color: Vec<f32>) -> Geom {
        Ellipse::new(radius, radius, color)
    }
}

