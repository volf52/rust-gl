use crate::graphics::{shapes::Rectangle, Shape};

#[derive(Debug, Clone)]
pub struct Dims {
    pub width: f32,
    pub height: f32,
}

pub trait Bounded: Shape {
    // Expects vertices in counter clockwise direction
    fn contains(&self, x: f32, y: f32) -> bool {
        let transform_mat = self.get_final_transformation_matrix();
        let (x_p, y_p) = transform_mat.inverse_affine_point(x, y);

        self.get_node()
            .borrow()
            .geom
            .vertices
            .chunks_exact(2)
            .map(|c| (c[0], c[1]))
            .collect::<Vec<_>>()
            .windows(2)
            .all(|window| {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];

                let a = y1 - y2;
                let b = x2 - x1;
                let c = -(a * x1 + b * y1);

                let d = a * x_p + b * y_p + c; // check direction of (x_p, y_p) with respect to the edge (x1, y1) <-> (x2, y2)

                d >= 0.0 // on the left
            })
    }

    // Get the bounding box without the transformations. Meant to be implemented by the shapes specifically in cases where
    // it would be more efficient to do so
    fn get_bounding_rect_inner(&self) -> Rectangle {
        let dims = self.get_node().borrow().geom.get_dims();

        Rectangle::new_at_origin(dims.width, dims.height, &vec![])
    }

    // Get the bounding rectangle with the transformations applied
    fn get_bounds(&self) -> Rectangle {
        let rect = self.get_bounding_rect_inner();
        rect.apply_transformations(&self.get_node().borrow().geom.u_mat);

        rect
    }

    // Check if point x,y exists within the bounding box
    fn contains_in_bounds(&self, x: f32, y: f32) -> bool {
        self.get_bounds().contains(x, y) // calls the `contains` method implemented for Rectangle in impl Rectangle{} block
    }
}
