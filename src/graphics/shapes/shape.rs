use crate::graphics::Geom;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Shape {
    fn get_geom(&self) -> Rc<RefCell<Geom>>;
    fn rotate(&self, angle: f32);
}
