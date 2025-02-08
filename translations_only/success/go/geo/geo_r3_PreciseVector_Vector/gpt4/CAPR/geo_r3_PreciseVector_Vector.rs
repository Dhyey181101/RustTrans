
use std::ops::Mul;

struct GeoR3PreciseVector {
    x: Box<u128>,
    y: Box<u128>,
    z: Box<u128>,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn new_geo_r3_vector(x: f64, y: f64, z: f64) -> GeoR3Vector {
    GeoR3Vector { x, y, z }
}

fn normalize(v: &GeoR3Vector) -> GeoR3Vector {
    let n2 = norm2(v);
    if n2 == 0.0 {
        new_geo_r3_vector(0.0, 0.0, 0.0)
    } else {
        mul(v, 1.0 / n2.sqrt())
    }
}

fn norm2(v: &GeoR3Vector) -> f64 {
    dot(v, v)
}

fn dot(v1: &GeoR3Vector, v2: &GeoR3Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn mul(v: &GeoR3Vector, m: f64) -> GeoR3Vector {
    new_geo_r3_vector(m * v.x, m * v.y, m * v.z)
}

fn vector(v: &GeoR3PreciseVector) -> GeoR3Vector {
    let x = *v.x as f64;
    let y = *v.y as f64;
    let z = *v.z as f64;
    normalize(&new_geo_r3_vector(x, y, z))
}
