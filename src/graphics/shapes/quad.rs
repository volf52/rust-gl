use std::cell::RefCell;
use std::rc::Rc;

use web_sys::WebGl2RenderingContext;

use crate::graphics::{Geom, Shape};
use crate::math::bounding_rect::Bounded;
use crate::math::Matrix;
use crate::textures::utils::TextureGen;

#[derive(Clone)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Rectangle {
    pub fn new_at(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color_or_texture: &impl TextureGen,
    ) -> Self {
        let right = width / 2.0;
        let left = -right;
        let top = height / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom, right, bottom].to_vec();

        let geom = Geom::new(
            &vertices,
            Matrix::translation(x, y),
            WebGl2RenderingContext::TRIANGLE_STRIP,
            4,
            color_or_texture,
        );

        Rectangle {
            width,
            height,
            geom: Rc::new(RefCell::new(geom)),
        }
    }

    pub fn new_at_origin(width: f32, height: f32, color_or_texture: &impl TextureGen) -> Self {
        Self::new_at(0.0, 0.0, width, height, color_or_texture)
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        let (c_x, c_y) = self.get_center();

        match (self.width, self.height) {
            t if t.0 <= 0.0 || t.1 <= 0.0 => false,
            t if ((x - c_x).abs() <= (t.0 / 2.0)) && ((y - c_y).abs() <= (t.1 / 2.0)) => true,
            _ => false,
        }
    }
}

impl Shape for Rectangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}

impl Bounded for Rectangle {
    fn contains(&self, x: f32, y: f32) -> bool {
        self.contains(x, y)
    }
}
