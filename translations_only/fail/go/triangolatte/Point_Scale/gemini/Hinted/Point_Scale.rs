
use std::f64::consts::PI;

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x.powi(2) + p.y.powi(2)).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}

struct Point {
    x: f64,
    y: f64,
}
