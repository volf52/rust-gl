use crate::graphics::shape::{Drawing, color_n_vertices};

use super::Geom;
use web_sys::WebGl2RenderingContext;

pub struct Rectangle {
    width: f32,
    height: f32,
    color: Vec<f32>,
}

pub struct Square {
    size: f32,
    color: Vec<f32>,
}

impl Drawing for Rectangle {
    fn draw_shape(&self) -> Geom {
        let right = self.width / 2.0;
        let left = -right;
        let top = self.height / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom, right, bottom].to_vec();
        let color = color_n_vertices(&self.color, 4);

        Geom {
            vertices,
            color,
            vertex_count: 4,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
        }
    }
}

impl Drawing for Square {
    fn draw_shape(&self) -> Geom {
        Rectangle{ width: self.size, height: self.size, color: self.color.clone()}.draw_shape()
    }
}