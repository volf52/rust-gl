use crate::{
    graphics::shape::{color_n_vertices, Drawing},
    math::Matrix,
};

use super::geom::Geom;
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

impl Rectangle {
    fn new(width: f32, height: f32, color: Vec<f32>) -> Geom {
        let right = width / 2.0;
        let left = -right;
        let top = height / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom, right, bottom].to_vec();
        let color = color_n_vertices(&color, 4);
        let matrix = Matrix::new();

        Geom {
            vertices,
            color,
            vertex_count: 4,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
            matrix,
        }
    }
}

impl Square {
    fn new(size: f32, color: Vec<f32>) -> Geom {
        Rectangle::new(size, size, color)
    }
}
