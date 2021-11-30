use crate::graphics::shapes::utils::{calc_n_vertices, color_n_vertices};
use crate::graphics::{Geom, Shape};
use crate::math::{BoundingRect, Matrix};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

pub struct RegularPolygon {
    pub x: i32,
    pub y: i32,

    pub radius: f32,
    pub sides: usize,

    geom: Rc<RefCell<Geom>>,
}

pub struct IrregularPolygon {
    pub x: i32,
    pub y: i32,

    pub width: f32,
    pub height: f32,
    pub sides: usize,

    geom: Rc<RefCell<Geom>>,
}

impl RegularPolygon {
    pub fn new(radius: f32, n_sides: usize, color: &[f32]) -> Self {
        Self::new_at(0, 0, radius, n_sides, color)
    }

    pub fn new_at(x: i32, y: i32, radius: f32, n_sides: usize, color: &[f32]) -> Self {
        let sides = n_sides.max(3);
        let vertices = calc_n_vertices(radius, radius, n_sides);
        let color_data = color_n_vertices(color, n_sides);

        let geom = Rc::new(RefCell::new(Geom {
            vertices,
            color: color_data,
            u_mat: Matrix::translation(x as f32, y as f32),
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: sides as i32,
        }));

        RegularPolygon {
            x,
            y,
            radius,
            sides,
            geom,
        }
    }
}

impl Shape for RegularPolygon {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        todo!()
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        todo!()
    }
}

impl IrregularPolygon {
    pub fn new(width: f32, height: f32, n_sides: usize, color: &[f32]) -> Self {
        Self::new_at(0, 0, width, height, n_sides, color)
    }

    pub fn new_from_path(vertices: Vec<f32>, color: &[f32]) -> Self {
        let color_data = color_n_vertices(color, vertices.len());
        let sides = vertices.len() / 2;

        let xs: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, e)| *e)
            .collect();

        let ys: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, e)| *e)
            .collect();

        let width = xs.iter().cloned().fold(f32::NAN, f32::max)
            - xs.iter().cloned().fold(f32::NAN, f32::min);
        let height = ys.iter().cloned().fold(f32::NAN, f32::max)
            - ys.iter().cloned().fold(f32::NAN, f32::min);

        let geom = Rc::new(RefCell::new(Geom {
            vertices,
            color: color_data,
            u_mat: Matrix::new(),
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: sides as i32,
        }));

        IrregularPolygon {
            x: 0,
            y: 0,
            width,
            height,
            sides,
            geom,
        }
    }

    pub fn new_at(x: i32, y: i32, width: f32, height: f32, n_sides: usize, color: &[f32]) -> Self {
        let sides = n_sides.max(3);
        let vertices = calc_n_vertices(width, height, n_sides);
        let color = color_n_vertices(color, n_sides);

        let geom = Rc::new(RefCell::new(Geom {
            vertices,
            color,
            u_mat: Matrix::translation(x as f32, y as f32),
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: sides as i32,
        }));

        IrregularPolygon {
            x,
            y,
            width,
            height,
            sides,
            geom,
        }
    }
}

impl Shape for IrregularPolygon {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        todo!()
    }

    fn contains(&self, _x: f32, _y: f32) -> bool {
        todo!()
    }
}
