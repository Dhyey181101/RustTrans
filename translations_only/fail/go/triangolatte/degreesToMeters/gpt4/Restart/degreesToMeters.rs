
use std::f64::consts::PI;

const ORIGIN_SHIFT: f64 = 2.0 * PI * 6378137.0 / 2.0;

struct Point {
    x: f64,
    y: f64,
}

fn degrees_to_meters(point: Box<Point>) -> Box<Point> {
    Box::new(Point {
        x: point.x * ORIGIN_SHIFT / 180.0,
        y: (point.y.to_radians().tan().ln() / (PI / 180.0)) * ORIGIN_SHIFT / 180.0,
    })
}
