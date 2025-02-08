
use std::f64;
use std::ops::{Mul, Add};

#[derive(Debug, Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone)]
struct GeoR3PreciseVector {
    x: Box<f64>,
    y: Box<f64>,
    z: Box<f64>,
}

fn vector(v: GeoR3PreciseVector) -> GeoR3Vector {
    GeoR3Vector {
        x: *v.x,
        y: *v.y,
        z: *v.z,
    }
}

fn normalize(v: GeoR3Vector) -> GeoR3Vector {
    let n2 = norm2(v);
    if n2 == 0.0 {
        return GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 };
    }
    mul(v, 1.0 / n2.sqrt())
}

fn norm2(v: GeoR3Vector) -> f64 {
    dot(v, v)
}

fn dot(v: GeoR3Vector, ov: GeoR3Vector) -> f64 {
    (v.x * ov.x) + (v.y * ov.y) + (v.z * ov.z)
}

fn mul(v: GeoR3Vector, m: f64) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x * m,
        y: v.y * m,
        z: v.z * m,
    }
}
