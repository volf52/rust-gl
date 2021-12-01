use crate::graphics::Geom;
use std::cell::RefCell;
use std::rc::Rc;



pub trait Shape {
    fn get_geom(&self) -> Rc<RefCell<Geom>>;

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

    fn move_by(&self, tx: f32, ty: f32) {
        self.translate(tx, ty);
    }
}


