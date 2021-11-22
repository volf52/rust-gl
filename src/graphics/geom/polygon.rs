use crate::{
    graphics::shape::{calc_n_vertices, color_n_vertices},
    math::Matrix,
};
use std::cmp;
use web_sys::WebGl2RenderingContext;

use super::geom::Geom;

pub struct IrregularPolygon {
    pub width: f32,
    pub height: f32,
    pub sides: usize,
    pub color: Vec<f32>,
}
pub struct RegularPolygon {
    pub radius: f32,
    pub sides: usize,
    pub color: Vec<f32>,
}

impl IrregularPolygon {
    fn new(sides: usize, width: f32, height: f32, color: Vec<f32>) -> Geom {
        let no_sides = cmp::max(3, sides);

        let vertices = calc_n_vertices(&width, &height, no_sides as u32);
        let color = color_n_vertices(&color, no_sides.clone() as usize);
        let matrix = Matrix::new();

        Geom {
            vertices,
            color,
            vertex_count: no_sides as i32,
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            matrix,
        }
    }

    pub fn new_from_path(vertices: Vec<f32>, color: &Vec<f32>) -> Geom {
        let color_data = color_n_vertices(color, vertices.len());
        let sides = vertices.len() / 2;

        let xs: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, e)| e.clone())
            .collect();

        let ys: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, e)| e.clone())
            .collect();

        let width =
            xs.iter().cloned().fold(0. / 0., f32::max) - xs.iter().cloned().fold(0. / 0., f32::min);
        let height =
            ys.iter().cloned().fold(0. / 0., f32::max) - ys.iter().cloned().fold(0. / 0., f32::min);

        Geom {
            vertices,
            color: color_data,
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: sides as i32,
            matrix: Matrix::new()
        }
    }
}

impl RegularPolygon {
    fn new(sides: usize, radius: f32, color: Vec<f32>) -> Geom {
        IrregularPolygon::new(sides, radius, radius, color)
    }
}
