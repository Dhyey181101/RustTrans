

use std::ops::{Add, Mul, Sub};

struct BigInt(i64);

impl BigInt {
    fn new(n: i64) -> BigInt {
        BigInt(n)
    }
    
    fn mul(&self, other: &BigInt) -> BigInt {
        BigInt(self.0 * other.0)
    }
    
    fn sub(&self, other: &BigInt) -> BigInt {
        BigInt(self.0 - other.0)
    }
}


struct GeoR3PreciseVector {
    x: Box<BigInt>,
    y: Box<BigInt>,
    z: Box<BigInt>,  
}

fn geo_r3_prec_mul(a: &BigInt, b: &BigInt) -> BigInt {
    a.mul(b)
}

fn geo_r3_prec_sub(a: &BigInt, b: &BigInt) -> BigInt {
    a.sub(b)
}

fn cross(v: &GeoR3PreciseVector, ov: &GeoR3PreciseVector) -> GeoR3PreciseVector {
    GeoR3PreciseVector {
        x: Box::new(geo_r3_prec_sub(&geo_r3_prec_mul(&v.y, &ov.z), &geo_r3_prec_mul(&v.z, &ov.y))),
        y: Box::new(geo_r3_prec_sub(&geo_r3_prec_mul(&v.z, &ov.x), &geo_r3_prec_mul(&v.x, &ov.z))),
        z: Box::new(geo_r3_prec_sub(&geo_r3_prec_mul(&v.x, &ov.y), &geo_r3_prec_mul(&v.y, &ov.x))),
    }
}

