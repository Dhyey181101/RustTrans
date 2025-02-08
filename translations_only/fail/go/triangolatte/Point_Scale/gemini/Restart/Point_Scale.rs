
use std::f64::consts::PI;

pub fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x.powi(2) + p.y.powi(2)).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
