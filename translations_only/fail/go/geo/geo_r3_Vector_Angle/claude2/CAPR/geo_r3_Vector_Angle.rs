
use std::f64::consts::PI;

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct Angle(f64);

const RADIAN: Angle = Angle(1.0);

fn angle(v: &Vector, ov: &Vector) -> Angle {
    Angle(dot(v, ov).atan2(norm(&cross(v, ov))) * RADIAN.0)
}

fn cross(v: &Vector, ov: &Vector) -> Vector {
    Vector {
        x: v.y * ov.z - v.z * ov.y, 
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x
    }
}

fn norm(v: &Vector) -> f64 {
    dot(v, v).sqrt()
}

fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z  
}

