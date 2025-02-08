
use std::f64;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn normalize(p: Point) -> Point {
    scale(p, 1.0)
}

fn scale(p: Point, f: f64) -> Point {
    let norm = f64::sqrt(p.x * p.x + p.y * p.y);
    if norm == 0.0 {
        Point { x: 0.0, y: 0.0 }
    } else {
        Point {
            x: p.x / norm * f,
            y: p.y / norm * f,
        }
    }
}
