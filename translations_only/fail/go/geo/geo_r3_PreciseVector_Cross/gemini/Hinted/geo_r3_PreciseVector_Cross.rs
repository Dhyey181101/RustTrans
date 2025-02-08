
use std::f64;

fn cross(v: geo_r3_precise_vector, ov: geo_r3_precise_vector) -> geo_r3_precise_vector {
    geo_r3_precise_vector {
        x: geo_r3_prec_sub(&v.y, &ov.z) - geo_r3_prec_mul(&v.z, &ov.y),
        y: geo_r3_prec_sub(&v.z, &ov.x) - geo_r3_prec_mul(&v.x, &ov.z),
        z: geo_r3_prec_sub(&v.x, &ov.y) - geo_r3_prec_mul(&v.y, &ov.x),
    }
}

fn geo_r3_prec_mul(a: &f64, b: &f64) -> f64 {
    a * b
}

fn geo_r3_prec_sub(a: &f64, b: &f64) -> f64 {
    a - b
}

#[derive(Debug)]
struct geo_r3_precise_vector {
    x: f64,
    y: f64,
    z: f64,
}
