
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        normalize(Point { y, x: -x }),
        normalize(Point { y: -y, x }),
    ]
}

fn normalize(p: Point) -> Point {
    scale(p, 1.0)  
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x.powi(2) + p.y.powi(2)).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}
