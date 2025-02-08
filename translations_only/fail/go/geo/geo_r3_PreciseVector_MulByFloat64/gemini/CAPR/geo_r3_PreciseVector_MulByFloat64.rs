
use std::ops::Mul;

#[derive(Debug)]
pub struct GeoR3PreciseVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GeoR3PreciseVector {
    pub fn mul_by_float64(&self, f: f64) -> GeoR3PreciseVector {
        self.mul(f)
    }

    pub fn mul(&self, f: f64) -> GeoR3PreciseVector {
        GeoR3PreciseVector {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}
