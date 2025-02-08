

use std::f64::consts::PI;

struct GeoR2Point {
    x: f64,
    y: f64,
}

fn normalize(p: &GeoR2Point) -> Box<GeoR2Point> {
    if p.x == 0.0 && p.y == 0.0 {
        return Box::new(GeoR2Point { x: 0.0, y: 0.0 });
    }
    let norm_p = norm(p);
    mul(Box::new(GeoR2Point { x: p.x, y: p.y }), 1.0 / norm_p)
}

fn norm(p: &GeoR2Point) -> f64 {
    (p.x.powi(2) + p.y.powi(2)).sqrt() 
}

fn mul(mut p: Box<GeoR2Point>, m: f64) -> Box<GeoR2Point> {
    p.x *= m;
    p.y *= m;
    p
}

