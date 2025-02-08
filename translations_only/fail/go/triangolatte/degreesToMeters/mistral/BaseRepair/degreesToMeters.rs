
extern crate core;
use core::f64;

const ORIGIN_SHIFT: f64 = (2.0 * f64::consts::PI * 6378137.0) / 2.0;
const MATH_PI: f64 = 3.14159265358979323846264338327950288419716939937510582097494459;

struct Point {
    x: f64,
    y: f64,
}

fn degrees_to_meters(point: Point) -> Point {
    Point {
        x: point.x * ORIGIN_SHIFT / 180.0,
        y: (MATH_PI / 180.0) * (point.y * MATH_PI / 180.0 + 90.0).ln().recip() * ORIGIN_SHIFT / 180.0,
    }
}
