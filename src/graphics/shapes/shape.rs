use crate::graphics::Geom;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Shape {
    fn get_geom(&self) -> Rc<RefCell<Geom>>;
    fn rotate(&self, angle_radians: f32) {
        let geom = self.get_geom();
        geom.borrow_mut().rotate(angle_radians);
    }
}
