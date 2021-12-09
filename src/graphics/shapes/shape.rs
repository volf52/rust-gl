use crate::graphics::Geom;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Shape {
    fn get_geom(&self) -> Rc<RefCell<Geom>>;

    // This center is not the absolute, rather relative to its parent's center
    fn get_center(&self) -> (f32, f32) {
        let mat = self.get_geom().borrow().u_mat.clone();

        (mat.tx, mat.ty)
    }

    fn get_scale(&self) -> (f32, f32) {
        let mat = self.get_geom().borrow().u_mat.clone();

        (mat.a, mat.d)
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
