
use std::ops::{Add, Mul};

const GEO_R3_PREC: usize = 32;

fn norm2(v: &geo_r3_PreciseVector) -> Box<i32> {
    dot(v, v)
}

fn dot(v: &geo_r3_PreciseVector, ov: &geo_r3_PreciseVector) -> Box<i32> {
    let x = prec_mul(&v.x, &ov.x);
    let y = prec_mul(&v.y, &ov.y);
    let z = prec_mul(&v.z, &ov.z);
    prec_add(&prec_add(&x, &y), &z)
}

fn prec_mul(a: &i32, b: &i32) -> Box<i32> {
    Box::new(a.clone() * b.clone())
}

fn prec_add(a: &i32, b: &i32) -> Box<i32> {
    Box::new(a.clone() + b.clone())
}

struct geo_r3_PreciseVector {
    x: i32,
    y: i32,
    z: i32,
}
