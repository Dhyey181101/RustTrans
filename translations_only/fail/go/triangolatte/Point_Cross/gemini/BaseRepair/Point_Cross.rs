
pub fn cross(p: Point, r: Point) -> f64 {
    p.x * r.y - p.y * r.x
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
