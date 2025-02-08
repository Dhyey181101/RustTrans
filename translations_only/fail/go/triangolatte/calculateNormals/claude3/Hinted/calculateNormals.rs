
use std::f64;

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        normalize(Point { x: y, y: -x }),
        normalize(Point { x: -y, y: x }),
    ]
}

fn normalize(p: Point) -> Point {
    p.scale(1.0)
}

fn scale(p: Point, f: f64) -> Point {
    let norm = f64::sqrt(p.x * p.x + p.y * p.y);
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn scale(&self, f: f64) -> Point {
        scale(*self, f)
    }
}
