#[derive(Clone)]
pub struct Rectangle {
    /**
     * @param x - The X coordinate of the upper-left corner of the rectangle
     * @param y - The Y coordinate of the upper-left corner of the rectangle
     * @param width - The overall width of the rectangle
     * @param height - The overall height of the rectangle
     */
    /** @default 0 */
    pub x: f64,

    /** @default 0 */
    pub y: f64,
    /** @default 0 */
    pub width: f64,

    /** @default 0 */
    pub height: f64,
}

impl Rectangle {
    /**
     * Checks whether the x and y coordinates given are contained within this Rectangle
     *
     * @param x - The X coordinate of the point to test
     * @param y - The Y coordinate of the point to test
     * @return Whether the x/y coordinates are within this Rectangle
     */
    fn contains(&self, x: f64, y: f64) -> bool {
        match (self.width, self.height) {
            t if t.0 <= 0.0 || t.1 <= 0.0 => return false,
            t if (x >= self.x && x < self.x + t.0) && (y >= self.y && y < self.y + t.1) => {
                return true
            }
            _ => return false,
        };
    }
}
