
use std::ops::{Add, Mul};

const GEO_R3_PREC: usize = 1024;

fn dot(v: &geo_r3_PreciseVector, ov: &geo_r3_PreciseVector) -> f64 {
    geo_r3_prec_add(
        &geo_r3_prec_mul(&v.x, &ov.x),
        &geo_r3_prec_add(
            &geo_r3_prec_mul(&v.y, &ov.y),
            &geo_r3_prec_mul(&v.z, &ov.z),
        ),
    )
}

fn geo_r3_prec_mul(a: &f64, b: &f64) -> f64 {
    a * b
}

fn geo_r3_prec_add(a: &f64, b: &f64) -> f64 {
    a + b
}

struct geo_r3_PreciseVector {
    x: f64,
    y: f64,
    z: f64,
}
