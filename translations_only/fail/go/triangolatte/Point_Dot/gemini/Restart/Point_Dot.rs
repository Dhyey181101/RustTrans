
pub fn dot(p: Point, r: Point) -> f64 {
    p.x * r.x + p.y * r.y
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
