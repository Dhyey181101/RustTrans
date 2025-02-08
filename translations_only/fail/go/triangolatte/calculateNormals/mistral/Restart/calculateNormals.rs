
use std::f64;
use std::boxed;

fn calculate_normals(x: f64, y: f64) -> [(f64, f64); 2] {
    [
        normalize((y, -x)),
        normalize((-y, x)),
    ]
}

fn normalize(p: (f64, f64)) -> (f64, f64) {
    let norm = f64::sqrt(p.0 * p.0 + p.1 * p.1);
    (p.0 / norm, p.1 / norm)
}

fn scale(f: f64, p: (f64, f64)) -> (f64, f64) {
    let norm = f64::sqrt(p.0 * p.0 + p.1 * p.1);
    (p.0 / norm * f, p.1 / norm * f)
}

type Point = (f64, f64);
