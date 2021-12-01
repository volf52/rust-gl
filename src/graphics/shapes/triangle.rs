use web_sys::WebGl2RenderingContext;

use crate::graphics::{Geom, Shape};
use crate::math::{BoundingRect, Matrix};
use crate::textures::utils::TextureGen;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Triangle {
    pub x: i32,
    pub y: i32,

    pub size: f32,
    geom: Rc<RefCell<Geom>>,
}

impl Triangle {
    pub fn new(x: i32, y: i32, size: f32, color_or_texture: &impl TextureGen) -> Self {
        let right = size / 2.0;
        let left = -right;
        let top = right;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom].to_vec();

        let geom = Rc::new(RefCell::new(Geom::new(
            &vertices,
            Matrix::translation(x as f32, y as f32),
            WebGl2RenderingContext::TRIANGLES,
            3,
            color_or_texture,
        )));

        Triangle { size, geom, x, y }
    }

    pub fn new_at_origin(size: f32, color_or_texture: &impl TextureGen) -> Self {
        Self::new(0, 0, size, color_or_texture)
    }
}

impl Shape for Triangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        todo!()
    }

    fn contains(&self, _x: f32, _y: f32) -> bool {
        todo!()
    }
}
