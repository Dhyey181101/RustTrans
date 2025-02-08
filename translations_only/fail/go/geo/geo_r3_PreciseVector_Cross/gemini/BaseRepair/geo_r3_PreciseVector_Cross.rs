
use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
struct GeoR3PreciseVector {
    x: Box<f64>,
    y: Box<f64>,
    z: Box<f64>,
}

impl GeoR3PreciseVector {
    fn cross(&self, other: &GeoR3PreciseVector) -> GeoR3PreciseVector {
        GeoR3PreciseVector {
            x: Box::new(
                geo_r3_prec_sub(
                    &geo_r3_prec_mul(&self.y, &other.z),
                    &geo_r3_prec_mul(&self.z, &other.y)
                )
            ),
            y: Box::new(
                geo_r3_prec_sub(
                    &geo_r3_prec_mul(&self.z, &other.x),
                    &geo_r3_prec_mul(&self.x, &other.z)
                )
            ),
            z: Box::new(
                geo_r3_prec_sub(
                    &geo_r3_prec_mul(&self.x, &other.y),
                    &geo_r3_prec_mul(&self.y, &other.x)
                )
            ),
        }
    }
}

fn geo_r3_prec_mul(a: &f64, b: &f64) -> f64 {
    a * b
}

fn geo_r3_prec_sub(a: &f64, b: &f64) -> f64 {
    a - b
}
