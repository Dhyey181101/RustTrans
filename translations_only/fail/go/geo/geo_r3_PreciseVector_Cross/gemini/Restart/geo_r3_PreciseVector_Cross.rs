
use std::ops::{Add, Mul, Sub};

fn cross(v: geo_r3_PreciseVector, ov: geo_r3_PreciseVector) -> geo_r3_PreciseVector {
    geo_r3_PreciseVector {
        X: (v.Y * ov.Z) - (v.Z * ov.Y),
        Y: (v.Z * ov.X) - (v.X * ov.Z),
        Z: (v.X * ov.Y) - (v.Y * ov.X),
    }
}

fn geo_r3_prec_mul(a: &f64, b: &f64) -> f64 {
    a * b
}

fn geo_r3_prec_sub(a: &f64, b: &f64) -> f64 {
    a - b
}

#[derive(Debug)]
struct geo_r3_PreciseVector {
    X: f64,
    Y: f64,
    Z: f64,
}
