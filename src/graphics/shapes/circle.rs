use crate::graphics::{
    scene_graph::{GraphEntity, GraphNode},
    shapes::Rectangle,
    Geom, Renderable, Transformable,
};
use crate::math::bounds::Bounded;
use crate::textures::utils::TextureGen;
use std::{cell::RefCell, rc::Rc};

pub struct Circle {
    pub radius: f32,

    node: Rc<RefCell<GraphNode>>,
}

impl Circle {
    pub fn new_at(x: f32, y: f32, radius: f32, color_or_texture: &impl TextureGen) -> Self {
        let vertex_count = 200;
        let geom = Geom::build_geom(x, y, radius, radius, vertex_count, color_or_texture);

        let node = GraphNode::for_shape(geom);

        Circle { radius, node }
    }

    pub fn new_at_origin(radius: f32, color_or_texture: &impl TextureGen) -> Self {
        Circle::new_at(0.0, 0.0, radius, color_or_texture)
    }
}

impl GraphEntity for Circle {
    fn get_node(&self) -> Rc<RefCell<GraphNode>> {
        self.node.clone()
    }
}

impl Transformable for Circle {}
impl Renderable for Circle {}

impl Bounded for Circle {
    fn contains(&self, x: f32, y: f32) -> bool {
        let (x_p, y_p) = self.node.borrow().geom.u_mat.inverse_affine_point(x, y);

        match self.radius {
            r2 if r2 <= 0.0 => false,
            r2 => {
                let dx = x_p.powi(2);
                let dy = y_p.powi(2);

                (dx + dy) <= r2.powi(2)
            }
        }
    }

    fn get_bounding_rect_inner(&self) -> Rectangle {
        let width_height = self.radius * 2.0;

        Rectangle::new_at_origin(width_height, width_height, &vec![])
    }
}
