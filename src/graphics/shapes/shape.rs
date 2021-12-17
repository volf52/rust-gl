use crate::graphics::Geom;
use crate::math::Matrix;
use crate::textures::utils::TextureGen;
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

pub trait Shape {
    fn get_geom(&self) -> Rc<RefCell<Geom>>;

    fn get_id(&self) -> uuid::Uuid {
        self.get_geom().borrow().id
    }

    fn get_parent_id(&self) -> Option<Uuid> {
        self.get_geom().borrow().parent_id
    }

    fn set_parent_id(&self, id: Uuid) {
        self.get_geom().borrow_mut().set_parent_id(id);
    }

    fn apply_transformations(&self, tranformation_mat: &Matrix) {
        self.get_geom().borrow_mut().u_mat = tranformation_mat.clone();
    }

    fn copy_transformations_from_geom(&self, geom: Rc<RefCell<Geom>>) {
        self.get_geom().borrow_mut().u_mat = geom.borrow().u_mat.clone();
    }

    fn copy_transformations(&self, other_shape: &impl Shape) {
        self.get_geom().borrow_mut().u_mat = other_shape.get_geom().borrow().u_mat.clone();
    }

    fn set_texture(&self, text_gen: &impl TextureGen) {
        self.get_geom().borrow_mut().set_texture(text_gen);
    }

    // This center is not the absolute, rather relative to its parent's center
    fn get_center(&self) -> (f32, f32) {
        let mat = self.get_geom().borrow().u_mat.clone();

        (mat.tx, mat.ty)
    }

    // Transformation funcs
    fn rotate(&self, angle_radians: f32) {
        let geom = self.get_geom();
        geom.borrow_mut().rotate(angle_radians);
    }

    fn rotate_deg(&self, angle_degrees: f32) {
        self.rotate(angle_degrees.to_radians());
    }

    fn translate(&self, tx: f32, ty: f32) {
        let geom = self.get_geom();
        geom.borrow_mut().translate(tx, ty);
    }

    fn scale(&self, x: f32, y: f32) {
        let geom = self.get_geom();
        geom.borrow_mut().scale(x, y);
    }

    // Move by tx,ty offset relative to the current position
    fn move_by(&self, tx: f32, ty: f32) {
        self.translate(tx, ty);
    }

    // Move to tx,ty position relative to the parent container's center
    fn move_to(&self, new_x: f32, new_y: f32) {
        let (center_x, center_y) = self.get_center();

        self.translate(new_x - center_x, new_y - center_y);
    }
}
