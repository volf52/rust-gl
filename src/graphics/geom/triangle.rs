use super::geom::Geom;
use crate::{graphics::shape::color_n_vertices, math::Matrix};
use web_sys::WebGl2RenderingContext;

pub struct Triangle {
    pub size: f32,
    pub color: Vec<f32>,
}

impl Triangle {
    pub fn new(size: f32, color: Vec<f32>) -> Geom {
        let right = size / 2.0;
        let left = -right;
        let top = size / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom].to_vec();
        let color = color_n_vertices(&color, 3);
        let matrix = Matrix::new();

        Geom {
            vertices,
            color,
            vertex_count: 3,
            mode: WebGl2RenderingContext::TRIANGLES,
            matrix,
        }
    }
}
