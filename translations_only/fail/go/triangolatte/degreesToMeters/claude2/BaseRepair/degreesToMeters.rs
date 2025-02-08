
use std::f64::consts::PI;

const ORIGIN_SHIFT: f64 = 2.0 * PI * 6378137.0 / 2.0;

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn degrees_to_meters(point: Point) -> Point {
    Point {
        x: point.x * ORIGIN_SHIFT / 180.0,
        y: (point.y.to_radians().tan() * ORIGIN_SHIFT / 180.0).ln() / (PI / 180.0),
    }
}
