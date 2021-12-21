use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
            idx_map: HashMap::new(),
        };

        self.node
            .borrow_mut()
            .add_child(Rc::new(RefCell::new(node)));

        shape.update_parent(Some(self.node.clone()));
    }

    pub fn remove_child(&mut self, child: &impl Shape) {
        self.node.borrow_mut().remove_child(child.get_id());
        child.update_parent(None);
    }

    pub fn add_container(&mut self, container: &Container) {
        self.node.borrow_mut().add_child(container.node.clone());
    }

    pub fn len(&self) -> usize {
        self.node.borrow().len()
    }

    pub fn contains(&self, id: uuid::Uuid) -> bool {
        self.node.borrow().contains(id)
    }
}

// `Shape` would be a misnomer here. Vector Space/Coord System would be better
impl Shape for Container {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.node.borrow().geom.clone()
    }
}
