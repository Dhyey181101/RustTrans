
use std::f64::consts::PI;

pub fn distance2(p: Point, r: Point) -> f64 {
    let dx = p.x - r.x;
    let dy = p.y - r.y;
    dx * dx + dy * dy
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
