use std::{cell::RefCell, rc::Rc};

use crate::graphics::scene_graph::GraphNode;
use crate::math::Matrix;
use crate::{core::application::Application, display::display_object::DisplayObject};

use super::{Geom, Shape};

pub struct Container {
    pub is_leaf: bool,

    pub geom: Rc<RefCell<Geom>>,

    pub children: Vec<Rc<RefCell<GraphNode>>>,
}

impl Default for Container {
    fn default() -> Self {
        let geom = Geom::default();

        Container {
            is_leaf: false,
            children: Vec::new(),
            geom: Rc::new(RefCell::new(geom)),
        }
    }
}

impl Shape for Container {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}

impl Container {
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

    pub fn add_shape(&mut self, shape: &impl Shape) {
        let node = GraphNode {
            is_leaf: true,
            geom: shape.get_geom(),
            children: Vec::new(),
        };

        self.children.push(Rc::new(RefCell::new(node)));
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn add_container(&mut self, container: &Container) {
        let node = GraphNode {
            is_leaf: false,
            geom: container.geom.clone(),
            children: container.children.clone(),
        };

        self.children.push(Rc::new(RefCell::new(node)));
    }
}
