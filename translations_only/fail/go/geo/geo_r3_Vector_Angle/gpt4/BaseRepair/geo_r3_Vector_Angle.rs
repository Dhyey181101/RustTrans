
use std::f64::consts::PI;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1Angle = f64;

const GEO_S1_RADIAN: GeoS1Angle = 1.0;

fn angle(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoS1Angle {
    (v.cross(ov).norm().atan2(v.dot(ov))) * GEO_S1_RADIAN
}

fn cross(v: &GeoR3Vector, ov: &GeoR3Vector) -> Box<GeoR3Vector> {
    Box::new(GeoR3Vector {
        x: v.y * ov.z - v.z * ov.y,
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x,
    })
}

fn norm(v: &GeoR3Vector) -> f64 {
    v.dot(v).sqrt()
}

fn dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

impl GeoR3Vector {
    fn cross(&self, ov: &GeoR3Vector) -> GeoR3Vector {
        *cross(self, ov)
    }

    fn norm(&self) -> f64 {
        norm(self)
    }

    fn dot(&self, ov: &GeoR3Vector) -> f64 {
        dot(self, ov)
    }
}
