use super::Geom;
use crate::math::Matrix;
use crate::{core::application::Application, display::display_object::DisplayObject};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct GraphNode {
    pub is_leaf: bool,
    pub geom: Rc<RefCell<Geom>>,
    pub children: Rc<RefCell<Vec<GraphNode>>>,
}

impl GraphNode {
    pub fn render(&self, app: &Application, parent_model_mat: &Matrix) {
        self.children.borrow_mut().iter().for_each(|child| {
            let updated_transform_mat =
                &Matrix::multiply(parent_model_mat, &self.geom.borrow_mut().u_mat);

            if child.is_leaf {
                let display_obj = DisplayObject::new(&app.ctx, child.geom.clone());
                display_obj.draw(&app.proj_mat, updated_transform_mat);
            } else {
                child.render(app, updated_transform_mat);
            }
        });
    }
}
