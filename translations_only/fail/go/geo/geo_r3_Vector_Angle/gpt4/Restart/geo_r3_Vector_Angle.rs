
use std::f64::consts::PI;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct GeoS1Angle(f64);

const GEO_S1_RADIAN: GeoS1Angle = GeoS1Angle(1.0);

fn angle(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoS1Angle {
    GeoS1Angle(v.cross(ov).norm().atan2(v.dot(ov))) * GEO_S1_RADIAN
}

impl GeoR3Vector {
    fn cross(&self, ov: &GeoR3Vector) -> Box<GeoR3Vector> {
        Box::new(GeoR3Vector {
            x: self.y * ov.z - self.z * ov.y,
            y: self.z * ov.x - self.x * ov.z,
            z: self.x * ov.y - self.y * ov.x,
        })
    }

    fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

impl std::ops::Mul<GeoS1Angle> for GeoS1Angle {
    type Output = GeoS1Angle;

    fn mul(self, rhs: GeoS1Angle) -> GeoS1Angle {
        GeoS1Angle(self.0 * rhs.0)
    }
}
