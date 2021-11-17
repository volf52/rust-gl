use crate::math::shapes::rectangle::Rectangle;

#[derive(Clone)]
struct Ellipse {
    /**
     * @param x - The X coordinate of the center of this ellipse
     * @param y - The Y coordinate of the center of this ellipse
     * @param halfWidth - The half width of this ellipse
     * @param halfHeight - The half height of this ellipse
     */

    /** @default 0 */
    x: f64,

    /** @default 0 */
    y: f64,

    /** @default 1 */
    width: f64,

    /** @default 0 */
    height: f64,
}

impl Ellipse {
    /**
     * Checks whether the x and y coordinates given are contained within this ellipse
     *
     * @param x - The X coordinate of the point to test
     * @param y - The Y coordinate of the point to test
     * @return Whether the x/y coords are within this ellipse
     */
    fn contains(&self, x: f64, y: f64) -> bool {
        let normx = ((x - self.x) / self.width).powf(2.0);
        let normy = ((y - self.y) / self.height).powf(2.0);

        match (self.width, self.height) {
            t if t.0 <= 0.0 || t.1 <= 0.0 => return false,
            _ => return normx + normy <= 1.0,
        }
    }

    /**
     * Returns the framing rectangle of the ellipse as a Rectangle object
     *
     * @return The framing rectangle
     */
    fn get_bounds(&self) -> Rectangle {
        return Rectangle {
            x: self.x - self.width,
            y: self.y - self.height,
            width: self.width,
            height: self.height,
        };
    }
}
