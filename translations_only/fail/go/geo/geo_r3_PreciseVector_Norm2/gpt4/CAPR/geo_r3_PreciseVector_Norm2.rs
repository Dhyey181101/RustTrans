
use std::str::FromStr;
use std::ops::{Add, Mul};

struct GeoR3PreciseVector {
    x: BigUint,
    y: BigUint,
    z: BigUint,
}

struct BigUint {
    value: Box<u128>,
}

impl BigUint {
    fn new(value: u128) -> BigUint {
        BigUint {
            value: Box::new(value),
        }
    }
}

fn norm2(v: &GeoR3PreciseVector) -> BigUint {
    dot(v, v)
}

fn dot(v1: &GeoR3PreciseVector, v2: &GeoR3PreciseVector) -> BigUint {
    geo_r3_prec_add(
        &geo_r3_prec_mul(&v1.x, &v2.x),
        &geo_r3_prec_add(
            &geo_r3_prec_mul(&v1.y, &v2.y),
            &geo_r3_prec_mul(&v1.z, &v2.z),
        ),
    )
}

fn geo_r3_prec_mul(a: &BigUint, b: &BigUint) -> BigUint {
    BigUint::new(*a.value * *b.value)
}

fn geo_r3_prec_add(a: &BigUint, b: &BigUint) -> BigUint {
    BigUint::new(*a.value + *b.value)
}

fn main() {}
