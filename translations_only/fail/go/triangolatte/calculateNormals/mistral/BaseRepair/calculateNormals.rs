
use std::f64;
use std::boxed;

struct Point {
    x: f64,
    y: f64,
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        normalize(Point { y, x }),
        normalize(Point { y: -y, x }),
    ]
}

fn normalize(mut p: Point) -> Point {
    let norm = (p.x * p.x + p.y * p.y).sqrt();
    p.x /= norm;
    p.y /= norm;
    p
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x * p.x + p.y * p.y).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}
