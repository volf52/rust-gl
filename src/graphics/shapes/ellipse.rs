use crate::graphics::{Geom, Shape};
use crate::math::bounding_rect::Bounded;
use crate::textures::utils::TextureGen;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Ellipse {
    pub width: f32,
    pub height: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Ellipse {
    pub fn new_at(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color_or_texture: &impl TextureGen,
    ) -> Self {
        let vertex_count = 200;
        let geom = Geom::build_geom(x, y, width, height, vertex_count, color_or_texture);

        Ellipse {
            width,
            height,
            geom,
        }
    }

    pub fn new_at_origin(width: f32, height: f32, color_or_texture: &impl TextureGen) -> Self {
        Self::new_at(0.0, 0.0, width, height, color_or_texture)
    }
}

impl Shape for Ellipse {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}

impl Bounded for Ellipse {
    // TODO: test
    fn contains(&self, x: f32, y: f32) -> bool {
        let (c_x, c_y) = self.get_center();

        match (self.width, self.height) {
            t if t.0 <= 0.0 || t.1 <= 0.0 => false,
            _ => {
                let norm_x = ((x - c_x) / self.width).powi(2);
                let norm_y = ((y - c_y) / self.height).powi(2);

                (norm_x + norm_y) <= 1.0
            }
        }
    }
}
