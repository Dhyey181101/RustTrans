
use std::f64::consts::PI;

pub fn normalize(p: geo_r2_Point) -> geo_r2_Point {
    if p.x == 0.0 && p.y == 0.0 {
        return p;
    }
    mul(p, 1.0 / norm(p))
}

pub fn norm(p: geo_r2_Point) -> f64 {
    (p.x.powi(2) + p.y.powi(2)).sqrt()
}

pub fn mul(p: geo_r2_Point, m: f64) -> geo_r2_Point {
    geo_r2_Point {
        x: p.x * m,
        y: p.y * m,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct geo_r2_Point {
    pub x: f64,
    pub y: f64,
}
