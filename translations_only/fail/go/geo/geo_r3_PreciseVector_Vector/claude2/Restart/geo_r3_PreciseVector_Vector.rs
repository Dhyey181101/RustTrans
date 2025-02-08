

use std::f64::consts::PI;

struct PreciseVector {
    x: Box<f64>,
    y: Box<f64>,
    z: Box<f64>,  
}

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn vector(v: PreciseVector) -> Vector {
    let x = *v.x;
    let y = *v.y;
    let z = *v.z;
    normalize(Vector { x, y, z })  
}

fn normalize(mut v: Vector) -> Vector {
    let n2 = norm2(&v);
    if n2 == 0.0 {
        return Vector { x: 0.0, y: 0.0, z: 0.0 };
    } else {
        mul(&mut v, 1.0 / (n2).sqrt());
    }
    v
}

fn norm2(v: &Vector) -> f64 {
    dot(v, v)
}

fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

fn mul(v: &mut Vector, m: f64) {
    v.x *= m;
    v.y *= m;
    v.z *= m;  
}

