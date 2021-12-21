use crate::graphics::{shapes::Rectangle, Shape};
pub trait Bounded: Shape {
    // Expects vertices in counter clockwise direction
    fn contains(&self, x: f32, y: f32) -> bool {
        let transform_mat = self.get_final_transformation_matrix();
        let (x_p, y_p) = transform_mat.inverse_affine_point(x, y);

        self.get_geom()
            .borrow()
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
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        self.get_geom()
            .borrow()
            .vertices
            .chunks_exact(2)
            .for_each(|chunk| {
                let x = chunk[0];
                let y = chunk[1];

                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            });

        let width = max_x - min_x;
        let height = max_y - min_y;

        Rectangle::new_at_origin(width, height, &vec![])
    }

    // Get the bounding rectangle with the transformations applied
    fn get_bounds(&self) -> Rectangle {
        let rect = self.get_bounding_rect_inner();
        rect.copy_transformations_from_geom(self.get_geom());

        rect
    }

    // Check if point x,y exists within the bounding box
    fn contains_in_bounds(&self, x: f32, y: f32) -> bool {
        self.get_bounds().contains(x, y) // calls the `contains` method implemented for Rectangle in impl Rectangle{} block
    }
}
