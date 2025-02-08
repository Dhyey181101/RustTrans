
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn distance(&self, other: GeoR3Vector) -> f64 {
        self.sub(other).norm()
    }

    fn sub(&self, other: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn norm(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    fn dot(&self, other: GeoR3Vector) -> f64 {
        (self.x * other.x) as f64 + (self.y * other.y) as f64 + (self.z * other.z) as f64
    }
}
