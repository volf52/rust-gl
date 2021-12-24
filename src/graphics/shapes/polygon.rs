use crate::graphics::{
    scene_graph::{GraphEntity, GraphNode},
    Geom, Shape,
};
use crate::math::bounds::{Bounded, BoundingDims};
use crate::textures::utils::TextureGen;

use std::{cell::RefCell, rc::Rc};

pub struct RegularPolygon {
    pub radius: f32,
    pub sides: usize,

    node: Rc<RefCell<GraphNode>>,
}

pub struct IrregularPolygon {
    pub width: f32,
    pub height: f32,
    pub sides: usize,

    node: Rc<RefCell<GraphNode>>,
}

impl RegularPolygon {
    pub fn new_at(
        x: f32,
        y: f32,
        radius: f32,
        n_sides: usize,
        color_or_texture: &impl TextureGen,
    ) -> Self {
        let sides = n_sides.max(3);
        let geom = Geom::build_geom(x, y, radius, radius, sides, color_or_texture);
        let node = GraphNode::for_shape(geom);

        RegularPolygon {
            radius,
            sides,
            node,
        }
    }

    pub fn new_at_origin(radius: f32, n_sides: usize, color_or_texture: &impl TextureGen) -> Self {
        Self::new_at(0.0, 0.0, radius, n_sides, color_or_texture)
    }
}

impl GraphEntity for RegularPolygon {
    fn get_node(&self) -> Rc<RefCell<GraphNode>> {
        self.node.clone()
    }
}

impl Shape for RegularPolygon {}
impl Bounded for RegularPolygon {}

impl IrregularPolygon {
    pub fn new_at(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        n_sides: usize,
        color_or_texture: &impl TextureGen,
    ) -> Self {
        let sides = n_sides.max(3);
        let geom = Geom::build_geom(x, y, width, height, sides, color_or_texture);
        let node = GraphNode::for_shape(geom);

        IrregularPolygon {
            width,
            height,
            sides,
            node,
        }
    }

    // path contains vertices in counter clockwise direction
    pub fn new_from_path(vertices: &[f32], color_or_texture: &impl TextureGen) -> Self {
        let sides = vertices.len() / 2;

        let bounding_dims = BoundingDims::from_vertices(vertices);
        let (width, height) = (bounding_dims.width, bounding_dims.height);

        let geom = Geom::build_geom(0.0, 0.0, width, height, sides, color_or_texture);
        let node = GraphNode::for_shape(geom);

        IrregularPolygon {
            width,
            height,
            sides,
            node,
        }
    }

    pub fn new_at_origin(
        width: f32,
        height: f32,
        n_sides: usize,
        color_or_texture: &impl TextureGen,
    ) -> Self {
        Self::new_at(0.0, 0.0, width, height, n_sides, color_or_texture)
    }
}

impl GraphEntity for IrregularPolygon {
    fn get_node(&self) -> Rc<RefCell<GraphNode>> {
        self.node.clone()
    }
}

impl Shape for IrregularPolygon {}
impl Bounded for IrregularPolygon {}
