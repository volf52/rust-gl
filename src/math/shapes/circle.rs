use crate::math::shapes::rectangle::Rectangle;

#[derive(Clone)]
struct Circle {
    /**
     * @param x - The X coordinate of the center of this circle
     * @param y - The Y coordinate of the center of this circle
     * @param radius - The radius of the circle
     */

    /** @default 0 */
    x: f64,

    /** @default 0 */
    y: f64,

    /** @default 1 */
    radius: f64,
}

impl Circle {
    /**
     * Checks whether the x and y coordinates given are contained within this circle
     *
     * @param x - The X coordinate of the point to test
     * @param y - The Y coordinate of the point to test
     * @return Whether the x/y coordinates are within this Circle
     */
    fn contains(&self, x: f64, y: f64) -> bool {
        let r2 = self.radius.powf(2.0);
        let dx = (self.x - x).powf(2.0);
        let dy = (self.y - y).powf(2.0);

        match (r2, dx, dy) {
            t if t.0 <= 0.0 => return false,
            _ => return dx + dy <= r2,
        }
    }
    /**
     * Returns the framing rectangle of the circle as a Rectangle object
     *
     * @return The framing rectangle
     */
    fn get_bounds(&self) -> Rectangle {
        return Rectangle {
            x: self.x - self.radius,
            y: self.y - self.radius,
            width: self.radius.powf(2.0),
            height: self.radius.powf(2.0),
        };
    }
}
