
use std::ops::{Add, Mul};

const GEO_R3_PREC: u32 = 1000;

fn norm2(v: &geo_r3_PreciseVector) -> i128 {
    dot(v, v)
}

fn dot(v: &geo_r3_PreciseVector, ov: &geo_r3_PreciseVector) -> i128 {
    geo_r3_prec_add(
        &geo_r3_prec_mul(&v.x, &ov.x),
        &geo_r3_prec_add(
            &geo_r3_prec_mul(&v.y, &ov.y),
            &geo_r3_prec_mul(&v.z, &ov.z),
        ),
    )
}

fn geo_r3_prec_mul(a: &i128, b: &i128) -> i128 {
    a.clone() * b.clone()
}

fn geo_r3_prec_add(a: &i128, b: &i128) -> i128 {
    a.clone() + b.clone()
}

struct geo_r3_PreciseVector {
    x: i128,
    y: i128,
    z: i128,
}
