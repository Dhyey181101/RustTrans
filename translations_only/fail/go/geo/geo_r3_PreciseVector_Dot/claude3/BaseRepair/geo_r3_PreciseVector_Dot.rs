
use std::ops::{Add, Mul};

const GEO_R3_PREC: u32 = 1024;

fn geo_r3_prec_mul(a: Box<f64>, b: Box<f64>) -> Box<f64> {
    Box::new((*a * *b).trunc() / 10.0_f64.powi(GEO_R3_PREC as i32) as f64)
}

fn geo_r3_prec_add(a: Box<f64>, b: Box<f64>) -> Box<f64> {
    Box::new((*a + *b).trunc() / 10.0_f64.powi(GEO_R3_PREC as i32) as f64)
}

struct GeoR3PreciseVector {
    x: Box<f64>,
    y: Box<f64>,
    z: Box<f64>,
}

fn geo_r3_precise_vector_dot(v1: Box<GeoR3PreciseVector>, v2: Box<GeoR3PreciseVector>) -> Box<f64> {
    geo_r3_prec_add(
        geo_r3_prec_mul(v1.x, v2.x),
        geo_r3_prec_add(
            geo_r3_prec_mul(v1.y, v2.y),
            geo_r3_prec_mul(v1.z, v2.z),
        ),
    )
}
