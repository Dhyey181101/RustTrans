
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

fn normalize(p: Point) -> Point {
    scale(p, 1.0)
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x.powi(2) + p.y.powi(2)).sqrt();
    if norm == 0.0 {
        panic!("Invalid input");
    }
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}
