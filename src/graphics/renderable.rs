use crate::{core::application::Application, display::display_object::DisplayObject, math::Matrix};

use super::scene_graph::GraphEntity;

pub trait Renderable: GraphEntity {
    fn render(&self, app: &Application, parent_model_mat: &Matrix) {
        self.get_node().borrow().render(app, parent_model_mat);
    }
}
