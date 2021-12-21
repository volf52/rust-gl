use super::Geom;
use crate::math::Matrix;
use crate::{core::application::Application, display::display_object::DisplayObject};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;

#[derive(Clone)]
pub struct GraphNode {
    pub is_leaf: bool,
    pub geom: Rc<RefCell<Geom>>,

    pub idx_map: HashMap<Uuid, usize>,
    pub children: Vec<Rc<RefCell<GraphNode>>>,
}

impl Default for GraphNode {
    fn default() -> Self {
        GraphNode {
            is_leaf: false,
            geom: Rc::new(RefCell::new(Geom::default())),
            children: Vec::new(),
            idx_map: HashMap::new(),
        }
    }
}

impl GraphNode {
    pub fn remove_child(&mut self, id: Uuid) -> bool {
        if let Some(idx) = self.idx_map.remove(&id) {
            self.children.remove(idx);

            return true;
        }

        false
    }

    pub fn get_id(&self) -> Uuid {
        self.geom.borrow_mut().id
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<GraphNode>>> {
        self.geom.borrow().parent.clone()
    }

    pub fn add_child(&mut self, node: Rc<RefCell<GraphNode>>) {
        let idx = self.children.len();
        let node_id = node.borrow().get_id();
        self.children.push(node);
        self.idx_map.insert(node_id, idx);
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn contains(&self, id: uuid::Uuid) -> bool {
        self.idx_map.get(&id).is_some()
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
