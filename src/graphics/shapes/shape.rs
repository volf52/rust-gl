use crate::graphics::shapes::Triangle;
use crate::graphics::Geom;
use crate::math::BoundingRect;
use crate::textures::utils::TextureOrColor;
use std::cell::RefCell;
use std::rc::Rc;

use super::{Ellipse, IrregularPolygon};

pub struct Dims {
    pub width: f32,
    pub height: f32,
}
pub trait Shape {
    fn get_geom(&self) -> Rc<RefCell<Geom>>;
    fn get_bounds(&self) -> BoundingRect;

    fn contains(&self, x: f32, y: f32) -> bool;

    fn contains_in_bounds(&self, x: f32, y: f32) -> bool {
        self.get_bounds().contains(x, y)
    }

    // Transformation funcs
    fn rotate(&self, angle_radians: f32) {
        let geom = self.get_geom();
        geom.borrow_mut().rotate(angle_radians);
    }

    fn rotate_deg(&self, angle_degrees: f32) {
        self.rotate(angle_degrees.to_radians());
    }

    fn translate(&self, tx: f32, ty: f32) {
        let geom = self.get_geom();
        geom.borrow_mut().translate(tx, ty);
    }

    fn set_texture(&self, mask: TextureOrColor) {
        let geom = self.get_geom();
        geom.borrow_mut().set_texture(mask);
    }

    fn get_dims(&self) -> Dims {
        self.get_geom().borrow().get_dims()
    }
}
