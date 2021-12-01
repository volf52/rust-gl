use crate::graphics::{Geom, Shape};
use crate::math::BoundingRect;
use crate::textures::utils::TextureGen;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Ellipse {
    pub x: i32,
    pub y: i32,
    pub width: f32,
    pub height: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Ellipse {
    pub fn new(
        x: i32,
        y: i32,
        width: f32,
        height: f32,
        color_or_texture: &impl TextureGen,
    ) -> Self {
        let vertex_count = 200;
        let geom = Geom::build_geom(
            x as f32,
            y as f32,
            width,
            height,
            vertex_count,
            color_or_texture,
        );

        Ellipse {
            x,
            y,
            width,
            height,
            geom,
        }
    }

    pub fn new_at_origin(width: f32, height: f32, color_or_texture: &impl TextureGen) -> Self {
        Self::new(0, 0, width, height, color_or_texture)
    }
}

impl Shape for Ellipse {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        let x_pos = (self.x as f32) - self.width;
        let y_pos = (self.y as f32) - self.height;

        BoundingRect::new(x_pos, y_pos, self.width, self.height)
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        match (self.width, self.height) {
            t if t.0 <= 0.0 || t.1 <= 0.0 => false,
            _ => {
                let norm_x = ((x - self.x as f32) / self.width).powi(2);
                let norm_y = ((y - self.y as f32) / self.height).powi(2);

                (norm_x + norm_y) <= 1.0
            }
        }
    }
}
