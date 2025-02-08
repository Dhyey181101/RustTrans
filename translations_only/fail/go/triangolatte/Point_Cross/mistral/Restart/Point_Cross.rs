
pub struct Point {
    x: f64,
    y: f64,
}

pub fn cross(p: &Point, r: &Point) -> f64 {
    p.x * r.y - p.y * r.x
}
