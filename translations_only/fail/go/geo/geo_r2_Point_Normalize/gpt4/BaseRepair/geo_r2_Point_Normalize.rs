
use std::f64;

struct GeoR2Point {
    x: f64,
    y: f64,
}

fn normalize(p: Box<GeoR2Point>) -> Box<GeoR2Point> {
    if p.x == 0.0 && p.y == 0.0 {
        return p;
    }
    let norm_value = norm(&p);
    mul(p, 1.0 / norm_value)
}

fn norm(p: &Box<GeoR2Point>) -> f64 {
    f64::hypot(p.x, p.y)
}

fn mul(p: Box<GeoR2Point>, m: f64) -> Box<GeoR2Point> {
    Box::new(GeoR2Point {
        x: m * p.x,
        y: m * p.y,
    })
}
