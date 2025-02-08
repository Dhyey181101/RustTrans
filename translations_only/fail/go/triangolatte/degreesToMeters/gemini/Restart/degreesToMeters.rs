
use std::f64::consts::PI;

const ORIGIN_SHIFT: f64 = 2.0 * PI * 6378137.0 / 2.0;
const MATH_PI: f64 = 3.14159265358979323846264338327950288419716939937510582097494459;

pub fn degrees_to_meters(point: Point) -> Point {
    Point {
        x: point.x * ORIGIN_SHIFT / 180.0,
        y: (MATH_PI / 180.0) * ORIGIN_SHIFT / 180.0 * (point.y + 90.0).tan().ln(),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
