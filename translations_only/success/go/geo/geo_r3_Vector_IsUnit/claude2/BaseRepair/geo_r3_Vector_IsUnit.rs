
use std::f64;

struct Vector {
    x: f64,
    y: f64,
    z: f64
}

fn is_unit(v: &Vector) -> bool {
    let epsilon = f64::EPSILON;
    (norm2(v) - 1.0).abs() <= epsilon
}

fn norm2(v: &Vector) -> f64 {
    dot(&v, &v)  
}

fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

