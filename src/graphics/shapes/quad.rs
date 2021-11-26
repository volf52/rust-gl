use std::cell::RefCell;
use std::rc::Rc;
use crate::graphics::{Geom, Shape};
use crate::graphics::shapes::utils::build;


#[derive(Clone)]
pub struct Rectangle {
    width: f32,
    height: f32,
    
    geom: Rc<RefCell<Geom>>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        let geom = build(width, height, 4);

        let rect = Rectangle {
            width,
            height,
            geom,
        };

        rect
    }
}

impl Shape for Rectangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}
