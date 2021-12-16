use crate::graphics::{shapes::Rectangle, Geom, Shape};
use crate::math::bounding_rect::Bounded;
use crate::textures::utils::TextureGen;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Ellipse {
    pub width: f32,  // center to horizontal edge
    pub height: f32, // center to horizontal edge

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
    // https://math.stackexchange.com/a/76463/525170
    fn contains(&self, x: f32, y: f32) -> bool {
        let (x_p, y_p) = self.geom.borrow().u_mat.inverse_affine_point(x, y);

        match (self.width, self.height) {
            t if t.0 <= 0.0 || t.1 <= 0.0 => false,
            _ => {
                let rx2 = self.width.powi(2);
                let ry2 = self.height.powi(2);

                let norm_x = x_p.powi(2) * ry2;
                let norm_y = y_p.powi(2) * rx2;

                (norm_x + norm_y) <= (rx2 * ry2)
            }
        }
    }

    fn get_bounding_rect_inner(&self) -> Rectangle {
        Rectangle::new_at_origin(self.width * 2.0, self.height * 2.0, &vec![])
    }
}
