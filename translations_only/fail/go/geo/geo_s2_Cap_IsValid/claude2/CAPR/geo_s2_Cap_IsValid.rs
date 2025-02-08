
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: f64,
}

impl GeoS2Cap {
    fn is_valid(&self) -> bool {
        is_unit(self.center.as_ref()) && self.radius <= GEO_S1_STRAIGHT_CHORD_ANGLE
    }
}

struct GeoS2Point {
    vector: Box<GeoR3Vector>,
}

fn is_unit(point: &GeoS2Point) -> bool {
    const EPSILON: f64 = 5e-14;
    (norm2(point.vector.as_ref()) - 1.0).abs() <= EPSILON
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn norm2(vector: &GeoR3Vector) -> f64 {
    dot(vector, vector)  
}

fn dot(v1: &GeoR3Vector, v2: &GeoR3Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

