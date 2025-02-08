
use std::f64;

const EPSILON: f64 = 5e-14;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn is_unit(&self) -> bool {
        (self.norm2() - 1.0).abs() <= EPSILON
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, other: &GeoR3Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
