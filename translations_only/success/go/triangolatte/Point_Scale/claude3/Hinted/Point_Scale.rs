
use std::f64;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn scale(p: Point, f: f64) -> Point {
    let norm = f64::sqrt(p.x * p.x + p.y * p.y);
    if norm == 0.0 {
        panic!("Input is invalid, crash gracefully");
    }
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}
