use std::cell::RefCell;
use std::rc::Rc;
use crate::graphics::{Geom, Shape};
use crate::graphics::shapes::utils::build;

pub struct Triangle {
    pub size: f32,
    geom: Rc<RefCell<Geom>>,
}

impl Triangle {
    pub fn new(size: f32) -> Self {
        let geom = build(size, size, 3);
        Triangle { size, geom }
    }
}

impl Shape for Triangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}


