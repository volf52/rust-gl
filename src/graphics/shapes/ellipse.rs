
use crate::graphics::{Geom, Shape};
use std::cell::RefCell;
use std::rc::Rc;
use crate::graphics::shapes::utils::build;

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
    pub fn new(width: f32, height: f32) -> Self {
        let vertex_count = 200;
        let geom = build(width, height, vertex_count);

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
    pub fn new(radius: f32) -> Self { 
        let vertex_count = 200;
        let geom = build(radius, radius, vertex_count);

        Circle { radius, geom }
    }
}

impl Shape for Circle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}
