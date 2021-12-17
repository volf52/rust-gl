use super::Geom;
use crate::math::Matrix;
use crate::{core::application::Application, display::display_object::DisplayObject};
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;

#[derive(Clone)]
pub struct GraphNode {
    pub is_leaf: bool,
    pub geom: Rc<RefCell<Geom>>,
    pub children: Vec<Rc<RefCell<GraphNode>>>,
}

impl Default for GraphNode {
    fn default() -> Self {
        GraphNode {
            is_leaf: false,
            geom: Rc::new(RefCell::new(Geom::default())),
            children: Vec::new(),
        }
    }
}

impl GraphNode {
    pub fn remove_child(&mut self, id: Uuid) -> bool {
        for (idx, node) in self.children.iter().enumerate() {
            if node.borrow().get_id() == id {
                node.borrow().geom.borrow_mut().parent_id = None;
                self.children.remove(idx);
                return true;
            }
        }

        false
    }

    // pub fn remove_child(&mut self, id: Uuid) {
    //     self.children.retain(|node| node.borrow().get_id() != id);
    // }

    pub fn get_id(&self) -> Uuid {
        self.geom.borrow().id
    }

    pub fn get_parent_id(&self) -> Option<Uuid> {
        self.geom.borrow().parent_id
    }

    pub fn set_parent_id(&self, id: Uuid) {
        self.geom.borrow_mut().set_parent_id(id);
    }

    pub fn add_child(&mut self, node: Rc<RefCell<GraphNode>>) {
        self.children.push(node);
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn render(&self, app: &Application, parent_model_mat: &Matrix) {
        self.children.iter().for_each(|child| {
            let updated_transform_mat = &parent_model_mat.mul(&self.geom.borrow().u_mat);

            let child_ref = child.borrow();

            if child_ref.is_leaf {
                let display_obj = DisplayObject::new(&app.ctx, child_ref.geom.clone());

                display_obj.draw(&app.proj_mat, updated_transform_mat);
            } else {
                child_ref.render(app, updated_transform_mat);
            }
        });
    }
}
