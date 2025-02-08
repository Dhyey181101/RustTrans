
use std::f64;

const GEO_S1_RADIAN: f64 = 1.0;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1Angle = f64;

fn angle(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoS1Angle {
    let cross_norm = v.cross(ov).norm();
    let dot = v.dot(ov);
    GEO_S1_RADIAN * f64::atan2(cross_norm, dot)
}

impl GeoR3Vector {
    fn cross(&self, ov: &GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.y * ov.z - self.z * ov.y,
            y: self.z * ov.x - self.x * ov.z,
            z: self.x * ov.y - self.y * ov.x,
        }
    }

    fn norm(&self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}
