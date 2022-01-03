use std::{cell::RefCell, rc::Rc};

use crate::core::application::Application;
use crate::graphics::scene_graph::GraphNode;
use crate::math::Matrix;

use super::scene_graph::GraphEntity;
use super::{Renderable, Transformable};

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
    pub fn add_child(&mut self, child: &impl Renderable) {
        self.node.borrow_mut().add_child(child.get_node());
        child.update_parent(Some(self.node.clone()))
    }

    pub fn remove_child(&mut self, child: &impl GraphEntity) {
        self.node.borrow_mut().remove_child(child.get_id());
        child.update_parent(None);
    }

    pub fn len(&self) -> usize {
        self.node.borrow().len()
    }

    pub fn contains(&self, id: uuid::Uuid) -> bool {
        self.node.borrow().contains(id)
    }
}

impl GraphEntity for Container {
    fn get_node(&self) -> Rc<RefCell<GraphNode>> {
        self.node.clone()
    }
}

impl Transformable for Container {}

impl Renderable for Container {}
