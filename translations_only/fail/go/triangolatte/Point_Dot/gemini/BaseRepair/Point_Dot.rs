
pub fn dot(p: Point, r: Point) -> f64 {
    p.x * r.x + p.y * r.y
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
