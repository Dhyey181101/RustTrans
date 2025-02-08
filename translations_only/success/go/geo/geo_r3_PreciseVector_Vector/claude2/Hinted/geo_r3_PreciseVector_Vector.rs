
use std::f64;

struct PreciseVector {
    x: i64,
    y: i64,
    z: i64,
}

struct Vector {
    x: f64,
    y: f64,
    z: f64,  
}

fn vector(v: PreciseVector) -> Vector {
    let x = v.x as f64;
    let y = v.y as f64;
    let z = v.z as f64;
    normalize(Vector { x, y, z })  
}

fn normalize(mut v: Vector) -> Vector {
    let norm2 = norm2(&v);
    if norm2 == 0.0 {
        return Vector { x: 0.0, y: 0.0, z: 0.0 };
    } else {
        mul(&mut v, 1.0 / f64::sqrt(norm2));
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

