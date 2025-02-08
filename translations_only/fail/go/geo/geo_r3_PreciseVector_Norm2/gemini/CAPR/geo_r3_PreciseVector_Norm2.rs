
use std::ops::{Add, Mul};

const GEO_R3_PREC: u32 = 32;

fn norm2(v: &geo_r3_PreciseVector) -> Box<f64> {
    dot(v, v)
}

fn dot(v: &geo_r3_PreciseVector, ov: &geo_r3_PreciseVector) -> Box<f64> {
    let x = prec_mul(&v.x, &ov.x);
    let y = prec_mul(&v.y, &ov.y);
    let z = prec_mul(&v.z, &ov.z);
    prec_add(&prec_add(&x, &y), &z)
}

fn prec_mul(a: &f64, b: &f64) -> Box<f64> {
    Box::new(a.clone() * b.clone())
}

fn prec_add(a: &f64, b: &f64) -> Box<f64> {
    Box::new(a.clone() + b.clone())
}

struct geo_r3_PreciseVector {
    x: Box<f64>,
    y: Box<f64>,
    z: Box<f64>,
}
