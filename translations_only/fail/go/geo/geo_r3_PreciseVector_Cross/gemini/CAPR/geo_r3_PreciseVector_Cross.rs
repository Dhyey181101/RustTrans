
use std::ops::{Sub, Mul};

#[derive(Debug)]
struct GeoR3PreciseVector {
    x: f64,
    y: f64,
    z: f64,
}

fn cross(v: &GeoR3PreciseVector, ov: &GeoR3PreciseVector) -> GeoR3PreciseVector {
    GeoR3PreciseVector {
        x: geo_r3_prec_sub(&v.y, &ov.z) * geo_r3_prec_mul(&v.z, &ov.y),
        y: geo_r3_prec_sub(&v.z, &ov.x) * geo_r3_prec_mul(&v.x, &ov.z),
        z: geo_r3_prec_sub(&v.x, &ov.y) * geo_r3_prec_mul(&v.y, &ov.x),
    }
}

fn geo_r3_prec_mul(a: &f64, b: &f64) -> f64 {
    a * b
}

fn geo_r3_prec_sub(a: &f64, b: &f64) -> f64 {
    a - b
}

fn main() {
    let v = GeoR3PreciseVector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let ov = GeoR3PreciseVector {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    let result = cross(&v, &ov);
    println!("{:?}", result);
}
