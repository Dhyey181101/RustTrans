
use std::f64::consts::PI;

fn is_unit(v: geo_r3_Vector) -> bool {
    const EPSILON: f64 = 5e-14;
    (norm2(v) - 1.0).abs() <= EPSILON
}

fn norm2(v: geo_r3_Vector) -> f64 {
    dot(v, v)
}

fn dot(v: geo_r3_Vector, ov: geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Debug, Clone, Copy)]
struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}
