
use std::ops::{Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct GeoR3PreciseVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GeoR3PreciseVector {
    pub fn mul_by_float64(&self, f: f64) -> Self {
        self.mul(&geo_r3_prec_float(f))
    }

    pub fn mul(&self, f: &f64) -> Self {
        GeoR3PreciseVector {
            x: geo_r3_prec_mul(self.x, *f),
            y: geo_r3_prec_mul(self.y, *f),
            z: geo_r3_prec_mul(self.z, *f),
        }
    }
}

impl Mul<f64> for GeoR3PreciseVector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        self.mul_by_float64(rhs)
    }
}

impl MulAssign<f64> for GeoR3PreciseVector {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.mul_by_float64(rhs);
    }
}

fn geo_r3_prec_float(f: f64) -> f64 {
    f
}

fn geo_r3_prec_mul(a: f64, b: f64) -> f64 {
    a * b
}
