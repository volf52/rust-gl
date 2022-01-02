use super::Geom;
use crate::math::Matrix;
use crate::{core::application::Application, display::display_object::DisplayObject};
use indexmap::IndexMap;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;

#[derive(Clone)]
pub struct GraphNode {
    pub id: Uuid,
    pub parent: Option<Rc<RefCell<GraphNode>>>,

    pub is_leaf: bool,
    pub geom: Geom,

    pub children: IndexMap<Uuid, Rc<RefCell<GraphNode>>>,
}

impl Default for GraphNode {
    fn default() -> Self {
        GraphNode {
            id: Uuid::new_v4(),
            parent: None,

            is_leaf: false,
            geom: Geom::default(),

            children: IndexMap::new(),
        }
    }
}

impl GraphNode {
    pub fn for_shape(geom: Geom) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(GraphNode {
            id: Uuid::new_v4(),
            parent: None,
            is_leaf: true,
            geom,
            children: IndexMap::new(),
        }))
    }

    pub fn remove_child(&mut self, id: Uuid) -> bool {
        self.children.shift_remove(&id).is_some()
    }

    pub fn add_child(&mut self, node: Rc<RefCell<GraphNode>>) {
        let node_id = node.borrow().id;

        self.children.insert(node_id, node);
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn contains(&self, id: uuid::Uuid) -> bool {
        self.children.get(&id).is_some()
    }

    pub fn get_final_transformation_matrix(&self) -> Matrix {
        let mut mat = self.geom.u_mat.clone();

        if let Some(p) = &self.parent {
            let mat2 = p.borrow().get_final_transformation_matrix();
            mat = mat2.mul(&mat);
        }

        mat
    }

    pub fn render(&self, app: &Application, parent_model_mat: &Matrix) {
        self.children.iter().for_each(|(_, child)| {
            let updated_transform_mat = &parent_model_mat.mul(&self.geom.u_mat);

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

pub trait GraphEntity {
    fn get_node(&self) -> Rc<RefCell<GraphNode>>;

    fn get_id(&self) -> Uuid {
        self.get_node().borrow().id
    }

    fn get_parent(&self) -> Option<Rc<RefCell<GraphNode>>> {
        self.get_node().borrow().parent.clone()
    }

    fn get_parent_id(&self) -> Option<Uuid> {
        let parent = self.get_node().borrow().parent.clone();

        parent.map(|p| p.borrow().id)
    }

    fn update_parent(&self, new_parent: Option<Rc<RefCell<GraphNode>>>) {
        let node = self.get_node();
        let node_id = node.borrow().id;

        if let Some(p) = &node.borrow().parent {
            p.borrow_mut().remove_child(node_id);
        }

        node.borrow_mut().parent = new_parent;
    }
}
