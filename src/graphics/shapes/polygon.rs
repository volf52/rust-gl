use crate::graphics::shapes::utils::{calc_n_vertices, color_n_vertices};
use crate::graphics::{Geom, Shape};
use crate::math::Matrix;
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
    pub fn new(x: i32, y: i32, radius: f32, n_sides: usize, color: &Vec<f32>) -> Self {
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

    pub fn new_at_origin(radius: f32, n_sides: usize, color: &Vec<f32>) -> Self {
        Self::new(0, 0, radius, n_sides, color)
    }
}

impl Shape for RegularPolygon {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}

impl IrregularPolygon {
    pub fn new(x: i32, y: i32, width: f32, height: f32, n_sides: usize, color: &Vec<f32>) -> Self {
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

    pub fn new_at_origin(width: f32, height: f32, n_sides: usize, color: &Vec<f32>) -> Self {
        Self::new(0, 0, width, height, n_sides, color)
    }
}

impl Shape for IrregularPolygon {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}
