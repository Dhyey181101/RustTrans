
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
        y: (MATH_PI / 180.0) * ORIGIN_SHIFT / 2.0 * (1.0 + (point.y + 90.0).tan()).ln(),
    }
}

fn main() {
    let point1 = Point {
        x: 3.746945975025107e+106,
        y: 6.814802878211214e-280,
    };
    degrees_to_meters(point1);

    let point2 = Point {
        x: 2.4161783066649913e+107,
        y: 5.198719998456673e-270,
    };
    degrees_to_meters(point2);
}
