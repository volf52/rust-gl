use std::{cell::RefCell, rc::Rc};

use crate::graphics::scene_graph::GraphNode;
use crate::math::Matrix;
use crate::{core::application::Application, display::display_object::DisplayObject};

use super::{Geom, Shape};

pub struct Container {
    pub geom: Rc<RefCell<Geom>>,

    pub children: Rc<RefCell<Vec<GraphNode>>>,
}

impl Default for Container {
    fn default() -> Self {
        let geom = Geom::default();

        Container {
            children: Rc::new(RefCell::new(Vec::new())),
            geom: Rc::new(RefCell::new(geom)),
        }
    }
}

impl Container {
    pub fn render(&self, app: &Application, parent_model_mat: &Matrix) {
        let t = self.children.borrow();
        self.children.borrow().iter().for_each(|child| {
            let updated_transform_mat =
                &Matrix::multiply(parent_model_mat, &self.geom.borrow().u_mat);

            if child.is_leaf {
                // shape render
                let display_obj = DisplayObject::new(&app.ctx, child.geom.clone());
                display_obj.draw(&app.proj_mat, updated_transform_mat);
            } else {
                // render children for the container
                child.render(app, updated_transform_mat);
            }
        });
    }

    pub fn add_shape(&mut self, shape: &impl Shape) {
        let node = GraphNode {
            is_leaf: true,
            geom: shape.get_geom(),
            children: Rc::new(RefCell::new(Vec::new())),
        };

        self.children.borrow_mut().push(node);
    }

    pub fn add_container(&mut self, container: &Container) {
        let node = GraphNode {
            is_leaf: false,
            geom: container.geom.clone(),
            children: container.children.clone(),
        };

        self.children.borrow_mut().push(node);
    }

    pub fn len(&self) -> usize {
        self.children.borrow().len()
    }
}

// `Shape` would be a misnomer here. Vector Space/Coord System would be better
impl Shape for Container {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}
