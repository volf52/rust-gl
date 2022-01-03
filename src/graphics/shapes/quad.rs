use std::{cell::RefCell, rc::Rc};
use web_sys::WebGl2RenderingContext;

use crate::graphics::{
    scene_graph::{GraphEntity, GraphNode},
    Geom, Renderable, Transformable,
};
use crate::math::{bounds::Bounded, Matrix};
use crate::textures::utils::TextureGen;

pub struct Rectangle {
    pub width: f32,
    pub height: f32,

    node: Rc<RefCell<GraphNode>>,
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

        let vertices = vec![left, top, right, top, left, bottom, right, bottom];

        let geom = Geom::new(
            &vertices,
            Matrix::translation(x, y),
            WebGl2RenderingContext::TRIANGLE_STRIP,
            4,
            color_or_texture,
        );

        let node = GraphNode::for_shape(geom);

        Rectangle {
            width,
            height,
            node,
        }
    }

    pub fn new_at_origin(width: f32, height: f32, color_or_texture: &impl TextureGen) -> Self {
        Self::new_at(0.0, 0.0, width, height, color_or_texture)
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        let (x_p, y_p) = self.node.borrow().geom.u_mat.inverse_affine_point(x, y);

        match (self.width, self.height) {
            t if t.0 <= 0.0 || t.1 <= 0.0 => false,
            t if (x_p.abs() <= t.0 / 2.0) && (y_p.abs() <= t.1 / 2.0) => true,
            _ => false,
        }
    }
}

impl GraphEntity for Rectangle {
    fn get_node(&self) -> Rc<RefCell<GraphNode>> {
        self.node.clone()
    }
}

impl Transformable for Rectangle {}
impl Renderable for Rectangle {}
impl Bounded for Rectangle {
    fn get_bounding_rect_inner(&self) -> Rectangle {
        Rectangle::new_at_origin(self.width, self.height, &vec![])
    }
}
