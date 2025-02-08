
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

fn normalize(v: Vector) -> Vector {
    let n2 = norm2(&v);
    if n2 == 0.0 {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    } else {
        mul(v, 1.0 / (n2).sqrt())
    }
}

fn norm2(v: &Vector) -> f64 {
    dot(v, v)
}

fn dot(v: &Vector, ov: &Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z  
}

fn mul(v: Vector, m: f64) -> Vector {
    Vector {
        x: m * v.x,
        y: m * v.y, 
        z: m * v.z
    }
}

