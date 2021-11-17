#[derive(Copy, Clone)]
struct IPointData {
    x: f64,
    y: f64,
}
pub struct Point {
    point: IPointData,
}

trait PointOperations {
    fn equals(&self, p: IPointData) -> bool;
    fn to_string(&self) -> String;
    fn clone(p: &Point) -> Point;
}

impl PointOperations for Point {
    fn to_string(&self) -> String {
        return format!("[@pixi/math:Point x={} y={}", self.point.x, self.point.y);
    }
    fn equals(&self, p: IPointData) -> bool {
        return (p.x == self.point.x) && (p.y == self.point.y);
    }
    fn clone(p: &Point) -> Point {
        return Point {
            point: p.point.clone(),
        };
    }
}
