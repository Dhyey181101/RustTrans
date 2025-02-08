
use std::f64::consts::SQRT_2;

#[derive(Debug, Clone, Copy)]
pub struct GeoR3PreciseVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct GeoR3Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GeoR3PreciseVector {
    pub fn vector(&self) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
        .normalize()
    }
}

impl GeoR3Vector {
    pub fn normalize(&self) -> GeoR3Vector {
        let n2 = self.norm2();
        if n2 == 0.0 {
            GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 }
        } else {
            self.mul(1.0 / SQRT_2)
        }
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }

    pub fn mul(&self, m: f64) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
        }
    }
}

