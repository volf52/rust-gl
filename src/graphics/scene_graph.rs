use super::Geom;
use crate::math::Matrix;
use crate::{core::application::Application, display::display_object::DisplayObject};
use std::{cell::RefCell, rc::Rc};

pub struct GraphNode {
    pub is_leaf: bool,
    pub geom: Rc<RefCell<Geom>>,
    pub children: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    pub fn render(&self, app: &Application, parent_model_mat: &Matrix) {
        self.children.iter().for_each(|child| {
            let borrowed = child.borrow();
            let updated_transform_mat =
                &Matrix::multiply(parent_model_mat, &self.geom.borrow().u_mat);

            if borrowed.is_leaf {
                // shape render
                let display_obj = DisplayObject::new(&app.ctx, borrowed.geom.clone());
                display_obj.draw(&app.proj_mat, updated_transform_mat);
            } else {
                // render children for the container
                borrowed.render(app, updated_transform_mat);
            }
        });
    }
}
