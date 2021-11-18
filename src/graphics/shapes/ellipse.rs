use crate::graphics::shapes::utils::{calc_n_vertices, color_n_vertices};
use crate::graphics::{Geom, Shape};
use crate::math::Matrix;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

pub struct Ellipse {
    pub width: f32,
    pub height: f32,

    geom: Rc<RefCell<Geom>>,
}
pub struct Circle {
    pub radius: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Ellipse {
    pub fn new(width: f32, height: f32, color: &Vec<f32>) -> Self {
        let vertex_count = 200;
        let vertices = calc_n_vertices(width, height, vertex_count);
        let color_data = color_n_vertices(color, vertex_count);

        let geom = Rc::new(RefCell::new(Geom {
            vertices,
            color: color_data,
            u_mat: Matrix::new(),
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: vertex_count as i32,
        }));

        Ellipse {
            width,
            height,
            geom,
        }
    }
}

impl Shape for Ellipse {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}

impl Circle {
    pub fn new(radius: f32, color: &Vec<f32>) -> Self {
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

        Circle { radius, geom }
    }
}

impl Shape for Circle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}
