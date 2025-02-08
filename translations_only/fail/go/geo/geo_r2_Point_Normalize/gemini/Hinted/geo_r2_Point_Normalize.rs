
use std::f64::consts::PI;

fn normalize(p: (f64, f64)) -> (f64, f64) {
    if p.0 == 0.0 && p.1 == 0.0 {
        return p;
    }
    let norm = (p.0 * p.0 + p.1 * p.1).sqrt();
    (p.0 / norm, p.1 / norm)
}

fn norm(p: (f64, f64)) -> f64 {
    (p.0 * p.0 + p.1 * p.1).sqrt()
}

fn mul(p: (f64, f64), m: f64) -> (f64, f64) {
    (p.0 * m, p.1 * m)
}

fn main() {
    let p = (2.5532292459299e-310, 1.4824512978972378e-104);
    let normalized = normalize(p);
    println!("{:?}", normalized);
}
