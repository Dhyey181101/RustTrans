
use std::f64::consts::PI;

pub fn is_unit(v: geo_r3_Vector) -> bool {
    const EPSILON: f64 = 5e-14;
    (norm2(v) - 1.0).abs() <= EPSILON
}

pub fn norm2(v: geo_r3_Vector) -> f64 {
    dot(v, v)
}

pub fn dot(v: geo_r3_Vector, ov: geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Copy, Clone)]
pub struct geo_r3_Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
