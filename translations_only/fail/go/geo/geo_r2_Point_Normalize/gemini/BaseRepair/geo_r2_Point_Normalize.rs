
use std::f64::consts::PI;

fn normalize(p: geo_r2_Point) -> geo_r2_Point {
    if p.x == 0.0 && p.y == 0.0 {
        return p;
    }
    mul(p, 1.0 / norm(p))
}

fn norm(p: geo_r2_Point) -> f64 {
    (p.x.powi(2) + p.y.powi(2)).sqrt()
}

fn mul(p: geo_r2_Point, m: f64) -> geo_r2_Point {
    geo_r2_Point { x: p.x * m, y: p.y * m }
}

#[derive(Copy, Clone)]
struct geo_r2_Point {
    x: f64,
    y: f64,
}
