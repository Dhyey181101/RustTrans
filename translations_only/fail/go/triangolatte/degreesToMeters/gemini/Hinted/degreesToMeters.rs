
use std::f64::consts::PI;

const ORIGIN_SHIFT: f64 = 2.0 * PI * 6378137.0 / 2.0;

pub fn degrees_to_meters(point: Point) -> Point {
    Point {
        x: point.x * ORIGIN_SHIFT / 180.0,
        y: (PI / 180.0) * ORIGIN_SHIFT / 180.0 * (point.y + 90.0).tan().ln(),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
