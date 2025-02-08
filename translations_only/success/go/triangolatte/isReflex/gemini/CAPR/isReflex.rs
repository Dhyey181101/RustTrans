
use std::f64::consts::PI;

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}
