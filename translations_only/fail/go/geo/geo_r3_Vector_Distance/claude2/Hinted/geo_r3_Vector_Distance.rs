

use std::f64::consts::SQRT_2;

struct Vector {
    x: f64,
    y: f64,
    z: f64
}

fn distance(v: &Vector, ov: &Vector) -> f64 {
    norm(&sub(v, ov))
}

fn sub(v: &Vector, ov: &Vector) -> Vector {
    Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z
    }
}

fn norm(v: &Vector) -> f64 {
    sqrt(dot(v, v))
}

fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z  
}

fn sqrt(x: f64) -> f64 {
    f64::sqrt(x)
}

