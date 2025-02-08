
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

fn scale(mut p: Point, f: f64) -> Point {
    let norm = (p.x*p.x + p.y*p.y).sqrt();
    if norm == 0.0 {
        panic!("Invalid input");
    }
    p.x /= norm;
    p.y /= norm;
    p.x *= f; 
    p.y *= f;
    p
}
