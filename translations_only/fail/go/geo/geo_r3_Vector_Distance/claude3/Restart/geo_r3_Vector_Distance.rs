
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
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

impl Add for GeoR3Vector {
    type Output = GeoR3Vector;

    fn add(self, other: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for GeoR3Vector {
    type Output = GeoR3Vector;

    fn sub(self, other: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for GeoR3Vector {
    type Output = GeoR3Vector;

    fn mul(self, other: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
