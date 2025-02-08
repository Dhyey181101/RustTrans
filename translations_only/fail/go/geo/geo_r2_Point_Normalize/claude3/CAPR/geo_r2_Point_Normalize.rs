
use std::f64::consts::SQRT_2;

#[derive(Clone, Copy)]
struct GeoR2Point {
    x: f64,
    y: f64,
}

fn normalize(p: GeoR2Point) -> GeoR2Point {
    if p.x == 0.0 && p.y == 0.0 {
        return p;
    }
    mul(p, 1.0 / norm(p))
}

fn norm(p: GeoR2Point) -> f64 {
    (p.x.powi(2) + p.y.powi(2)).sqrt()
}

fn mul(p: GeoR2Point, m: f64) -> GeoR2Point {
    GeoR2Point {
        x: p.x * m,
        y: p.y * m,
    }
}
