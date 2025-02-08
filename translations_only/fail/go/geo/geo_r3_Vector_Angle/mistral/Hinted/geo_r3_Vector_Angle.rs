

use std::f64;
use std::ops::Mul;

const GEO_S1_RADIAN: f64 = 1.0;

#[derive(Copy, Clone, Debug)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn cross(&self, ov: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.y * ov.z - self.z * ov.y,
            y: self.z * ov.x - self.x * ov.z,
            z: self.x * ov.y - self.y * ov.x,
        }
    }

    fn norm(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    fn dot(&self, ov: GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

#[derive(Copy, Clone, Debug)]
struct GeoS1Angle(f64);

fn angle(v: GeoR3Vector, ov: GeoR3Vector) -> GeoS1Angle {
    let cross_norm = v.cross(ov).norm();
    GeoS1Angle(cross_norm.atan2(v.dot(ov)) * GEO_S1_RADIAN)
}

