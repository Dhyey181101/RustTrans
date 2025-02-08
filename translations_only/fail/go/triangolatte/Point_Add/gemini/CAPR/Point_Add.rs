
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn add(p: Point, r: Point) -> Point {
    Point {
        x: p.x + r.x,
        y: p.y + r.y,
    }
}
