
use std::f64::consts::PI;

const ORIGIN_SHIFT: f64 = 2.0 * PI * 6378137.0 / 2.0;
const MATH_PI: f64 = 3.14159265358979323846264338327950288419716939937510582097494459;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn degrees_to_meters(point: Point) -> Point {
    let x = point.x * ORIGIN_SHIFT / 180.0;
    let y = (((90.0 + point.y) * MATH_PI / 360.0).tan().ln() / (MATH_PI / 180.0)) * ORIGIN_SHIFT / 180.0;

    Point { x, y }
}
