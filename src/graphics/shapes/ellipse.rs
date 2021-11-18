use crate::graphics::shapes::utils::{calc_n_vertices, color_n_vertices};
use crate::graphics::{Geom, Shape};
use crate::math::Matrix;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

pub struct Ellipse {
    pub x: i32,
    pub y: i32,
    pub width: f32,
    pub height: f32,

    geom: Rc<RefCell<Geom>>,
}
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Ellipse {
    pub fn new(x: i32, y: i32, width: f32, height: f32, color: &Vec<f32>) -> Self {
        let vertex_count = 200;
        let vertices = calc_n_vertices(width, height, vertex_count);
        let color_data = color_n_vertices(color, vertex_count);

        let geom = Rc::new(RefCell::new(Geom {
            vertices,
            color: color_data,
            u_mat: Matrix::translation(x as f32, y as f32),
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: vertex_count as i32,
        }));

        Ellipse {
            x,
            y,
            width,
            height,
            geom,
        }
    }

    pub fn new_at_origin(width: f32, height: f32, color: &Vec<f32>) -> Self {
        Self::new(0, 0, width, height, color)
    }
}

impl Shape for Ellipse {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: f32, color: &Vec<f32>) -> Self {
        let vertex_count = 200;
        let vertices = calc_n_vertices(radius, radius, vertex_count);
        let color_data = color_n_vertices(color, vertex_count);

        let geom = Rc::new(RefCell::new(Geom {
            vertices,
            color: color_data,
            u_mat: Matrix::new(),
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: vertex_count as i32,
        }));

        Circle { x, y, radius, geom }
    }

    pub fn new_at_origin(radius: f32, color: &Vec<f32>) -> Self {
        Self::new(0, 0, radius, color)
    }
}

impl Shape for Circle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}
