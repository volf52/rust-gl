use std::{cell::RefCell, rc::Rc};
use web_sys::WebGl2RenderingContext;

use crate::graphics::{
    scene_graph::{GraphEntity, GraphNode},
    Geom, Shape,
};
use crate::math::{bounds::Bounded, Matrix};
use crate::textures::utils::TextureGen;

pub struct Triangle {
    pub size: f32,

    node: Rc<RefCell<GraphNode>>,
}

impl Triangle {
    pub fn new(x: f32, y: f32, size: f32, color_or_texture: &impl TextureGen) -> Self {
        let right = size / 2.0;
        let left = -right;
        let top = right;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom].to_vec();

        let geom = Geom::new(
            &vertices,
            Matrix::translation(x, y),
            WebGl2RenderingContext::TRIANGLES,
            3,
            color_or_texture,
        );

        let node = GraphNode::for_shape(geom);

        Triangle { size, node }
    }

    pub fn new_at_origin(size: f32, color_or_texture: &impl TextureGen) -> Self {
        Self::new(0.0, 0.0, size, color_or_texture)
    }
}

impl GraphEntity for Triangle {
    fn get_node(&self) -> Rc<RefCell<GraphNode>> {
        self.node.clone()
    }
}

impl Shape for Triangle {}

impl Bounded for Triangle {}
