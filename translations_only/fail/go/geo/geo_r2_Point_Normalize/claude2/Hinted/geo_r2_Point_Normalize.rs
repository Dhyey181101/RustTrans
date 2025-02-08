
use std::f64::consts::PI;

struct GeoR2Point {
    x: f64,
    y: f64,
}

fn normalize(p: Box<GeoR2Point>) -> Box<GeoR2Point> {
    let p = *p;
    if p.x == 0.0 && p.y == 0.0 {
        return Box::new(p);
    }

    let norm = norm(&p);
    mul(p, 1.0 / norm)  
}

fn norm(p: &GeoR2Point) -> f64 {
    (p.x.powi(2) + p.y.powi(2)).sqrt()
}

fn mul(p: GeoR2Point, m: f64) -> Box<GeoR2Point> {
    Box::new(GeoR2Point {
        x: p.x * m,
        y: p.y * m,
    })
}
