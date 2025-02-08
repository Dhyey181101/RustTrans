
use std::f64::consts::PI;

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct Angle(f64);

fn angle(v: &Vector, ov: &Vector) -> f64 {
    let cross_result = cross(v, ov);
    let norm_result = norm(&cross_result);
    let dot_result = dot(v, ov);
    let atan2_result = dot_result.atan2(norm_result);
    atan2_result * PI
}

fn cross(v: &Vector, ov: &Vector) -> Vector {
    Vector {
        x: v.y * ov.z - v.z * ov.y,
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x,
    }
}

fn norm(v: &Vector) -> f64 {
    dot(v, v).sqrt()
}

fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z   
}

