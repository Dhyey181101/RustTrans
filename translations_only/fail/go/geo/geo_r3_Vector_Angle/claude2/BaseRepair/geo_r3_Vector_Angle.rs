
use std::f64::consts::PI;
use std::f64;

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct Angle(f64);

const RADIAN: Angle = Angle(1.0);

fn angle(v: &Vector, ov: &Vector) -> Angle {
    Angle(f64::atan2(norm(&cross(v, ov)), dot(v, ov)) * RADIAN.0)
}

fn cross(v: &Vector, ov: &Vector) -> Vector {
    Vector {
        x: v.y * ov.z - v.z * ov.y,
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x,
    }
}

fn norm(v: &Vector) -> f64 {
    f64::sqrt(dot(v, v))  
}

fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

