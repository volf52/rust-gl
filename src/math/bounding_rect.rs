use crate::graphics::{shapes::Rectangle, Shape};

pub trait Bounded: Shape {
    // Check if x,y is a point within the shape. Will fallback to checking it within the bounding box if not implemented
    fn contains(&self, x: f32, y: f32) -> bool {
        self.get_bounds().contains(x, y)
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
            .chunks(2)
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
