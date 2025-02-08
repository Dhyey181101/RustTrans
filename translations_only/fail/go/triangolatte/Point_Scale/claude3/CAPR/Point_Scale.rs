
use std::f64::consts::SQRT_2;

struct Point {
    x: f64,
    y: f64,
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x * p.x + p.y * p.y).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}
