
use std::boxed;

struct Point {
    x: f64,
    y: f64,
}

fn dot(p: Point, r: Point) -> f64 {
    p.x * r.x + p.y * r.y
}
