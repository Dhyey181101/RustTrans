
use std::boxed::Box;

mod num_bigint {
    pub struct BigInt;
    
    impl BigInt {
        pub fn max_bits() -> u32 {
            0
        }
        
        pub fn set_precision(&self, _: u32) -> &BigInt {
            &BigInt
        }
        
        pub fn mul(&self, _: &BigInt) -> BigInt {
            BigInt
        }
        
        pub fn add(&self, _: &BigInt) -> BigInt {
            BigInt
        }
    }
}

pub struct GeoR3PreciseVector {
    pub x: Box<num_bigint::BigInt>,
    pub y: Box<num_bigint::BigInt>,
    pub z: Box<num_bigint::BigInt>,
}

pub fn geo_r3_prec_mul(a: &num_bigint::BigInt, b: &num_bigint::BigInt) -> Box<num_bigint::BigInt> {
    Box::new(a.mul(b))  
}

pub fn geo_r3_prec_add(a: &Box<num_bigint::BigInt>, b: &Box<num_bigint::BigInt>) -> Box<num_bigint::BigInt> {
    Box::new(a.add(&*b)) 
}

pub fn geo_r3_precise_vector_dot(v1: &GeoR3PreciseVector, v2: &GeoR3PreciseVector) -> Box<num_bigint::BigInt> {
    let tmp1 = geo_r3_prec_mul(&v1.x, &v2.x);
    let tmp2 = geo_r3_prec_mul(&v1.y, &v2.y);
    let tmp3 = geo_r3_prec_mul(&v1.z, &v2.z);
    geo_r3_prec_add(&tmp1, &geo_r3_prec_add(&tmp2, &tmp3))
}

