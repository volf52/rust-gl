use super::Geom;
use crate::graphics::shape::{Drawing, color_n_vertices};
use web_sys::WebGl2RenderingContext;

pub struct Triangle {
    pub size: f32,
    pub color: Vec<f32>,
}

impl Drawing for Triangle {
    fn draw_shape(&self) -> Geom {
        let right = self.size / 2.0;
        let left = -right;
        let top = self.size / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom].to_vec();
        let color = color_n_vertices(&self.color, 3);

        Geom {
            vertices,
            color,
            vertex_count: 3,
            mode: WebGl2RenderingContext::TRIANGLES,
        }
    }
}
