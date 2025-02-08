
use std::f64::consts::PI;

#[derive(Copy, Clone)]
struct Vector {
    x: f64,
    y: f64, 
    z: f64
}

#[derive(Copy, Clone)]
struct Angle(f64);

const RADIAN: Angle = Angle(1.0);

fn cross(v: Vector, ov: Vector) -> Vector {
    Vector {
        x: v.y * ov.z - v.z * ov.y,
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x
    }
}

fn norm(v: Vector) -> f64 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

fn dot(v: Vector, ov: Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z   
}

fn angle(v: Vector, ov: Vector) -> Angle {
    let n = norm(v);
    let on = norm(ov);
    if n == 0.0 || on == 0.0 {
        panic!("Invalid input");
    }

    let cos = dot(v, ov) / (n * on);
    let rad = cos.acos();
    Angle(rad * RADIAN.0)
}
