
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn distance(&self, other: GeoR3Vector) -> f64 {
        sub(*self, other).norm()
    }

    fn norm(&self) -> f64 {
        dot(*self, *self).sqrt()
    }
}

fn sub(v1: GeoR3Vector, v2: GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

fn dot(v1: GeoR3Vector, v2: GeoR3Vector) -> f64 {
    (v1.x * v2.x) as f64 + (v1.y * v2.y) as f64 + (v1.z * v2.z) as f64
}
