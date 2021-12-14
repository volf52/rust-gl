use crate::math::bounding_rect::Bounded;
use std::{cell::RefCell, rc::Rc};
use web_sys::WebGl2RenderingContext;

use crate::graphics::{Geom, Shape};
use crate::math::Matrix;
use crate::textures::utils::TextureGen;

pub struct Triangle {
    pub size: f32,
    geom: Rc<RefCell<Geom>>,
}

impl Triangle {
    pub fn new(x: f32, y: f32, size: f32, color_or_texture: &impl TextureGen) -> Self {
        let right = size / 2.0;
        let left = -right;
        let top = right;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom].to_vec();

        let geom = Rc::new(RefCell::new(Geom::new(
            &vertices,
            Matrix::translation(x, y),
            WebGl2RenderingContext::TRIANGLES,
            3,
            color_or_texture,
        )));

        Triangle { size, geom }
    }

    pub fn new_at_origin(size: f32, color_or_texture: &impl TextureGen) -> Self {
        Self::new(0.0, 0.0, size, color_or_texture)
    }
}

impl Shape for Triangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}

impl Bounded for Triangle {}
