use crate::graphics::scene_graph::{GraphEntity, GraphNode};
use crate::math::Matrix;
use crate::textures::utils::TextureGen;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Shape: GraphEntity {
    fn get_model_matrix(&self) -> Matrix {
        self.get_node().borrow().geom.u_mat.clone()
    }

    fn get_final_transformation_matrix(&self) -> Matrix {
        self.get_node()
            .borrow_mut()
            .get_final_transformation_matrix()
    }

    fn apply_transformations(&self, tranformation_mat: &Matrix) {
        self.get_node().borrow_mut().geom.u_mat = tranformation_mat.clone();
    }

    fn copy_transformations_from_node(&self, node: Rc<RefCell<GraphNode>>) {
        self.get_node().borrow_mut().geom.u_mat = node.borrow().geom.u_mat.clone();
    }

    fn copy_transformations(&self, other_shape: &impl Shape) {
        self.get_node().borrow_mut().geom.u_mat =
            other_shape.get_node().borrow().geom.u_mat.clone();
    }

    fn set_texture(&self, text_gen: &impl TextureGen) {
        self.get_node().borrow_mut().geom.set_texture(text_gen);
    }

    // This center is not the absolute, rather relative to its parent's center
    fn get_center(&self) -> (f32, f32) {
        let mat = self.get_node().borrow().geom.u_mat.clone();

        (mat.tx, mat.ty)
    }

    // Transformation funcs
    fn rotate(&self, angle_radians: f32) {
        self.get_node().borrow_mut().geom.rotate(angle_radians);
    }

    fn rotate_deg(&self, angle_degrees: f32) {
        self.get_node()
            .borrow_mut()
            .geom
            .rotate(angle_degrees.to_radians());
    }

    fn translate(&self, tx: f32, ty: f32) {
        self.get_node().borrow_mut().geom.translate(tx, ty);
    }

    fn scale(&self, x: f32, y: f32) {
        self.get_node().borrow_mut().geom.scale(x, y);
    }

    // Move by tx,ty offset relative to the current position
    fn move_by(&self, tx: f32, ty: f32) {
        self.get_node().borrow_mut().geom.translate(tx, ty);
    }

    // Move to tx,ty position relative to the parent container's center
    fn move_to(&self, new_x: f32, new_y: f32) {
        let (center_x, center_y) = self.get_center();

        self.get_node()
            .borrow_mut()
            .geom
            .translate(new_x - center_x, new_y - center_y);
    }
}
