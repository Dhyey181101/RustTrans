
use std::ops::{Add, Mul};

fn main() {}

struct GeoR3PreciseVector {
    x: Box<u128>,
    y: Box<u128>,
    z: Box<u128>,
}

fn dot(v: &GeoR3PreciseVector, ov: &GeoR3PreciseVector) -> Box<u128> {
    Box::new(
        *geo_r3_prec_add(
            &*geo_r3_prec_mul(&v.x, &ov.x),
            &*geo_r3_prec_add(
                &*geo_r3_prec_mul(&v.y, &ov.y),
                &*geo_r3_prec_mul(&v.z, &ov.z),
            ),
        ),
    )
}

fn geo_r3_prec_mul(a: &u128, b: &u128) -> Box<u128> {
    Box::new(a.mul(b))
}

fn geo_r3_prec_add(a: &u128, b: &u128) -> Box<u128> {
    Box::new(a.add(b))
}
