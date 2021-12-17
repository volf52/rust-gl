use std::{cell::RefCell, rc::Rc};

use crate::core::application::Application;
use crate::graphics::scene_graph::GraphNode;
use crate::math::Matrix;

use super::{Geom, Shape};

pub struct Container {
    pub node: Rc<RefCell<GraphNode>>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            node: Rc::new(RefCell::new(GraphNode::default())),
        }
    }
}

impl Container {
    pub fn render(&self, app: &Application, parent_model_mat: &Matrix) {
        self.node.borrow().render(app, parent_model_mat);
    }

    pub fn add_shape(&mut self, shape: &impl Shape) {
        let node = GraphNode {
            is_leaf: true,
            geom: shape.get_geom(),
            children: Vec::new(),
        };

        self.node
            .borrow_mut()
            .add_child(Rc::new(RefCell::new(node)));

        shape.set_parent_id(self.node.borrow().get_id());
    }

    pub fn remove_child(&mut self, child: &impl Shape) {
        let shape_id = child.get_id();

        self.node.borrow_mut().remove_child(shape_id);
    }

    pub fn add_container(&mut self, container: &Container) {
        self.node.borrow_mut().add_child(container.node.clone());
    }

    pub fn len(&self) -> usize {
        self.node.borrow().len()
    }
}

// `Shape` would be a misnomer here. Vector Space/Coord System would be better
impl Shape for Container {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.node.borrow().geom.clone()
    }
}
