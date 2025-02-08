
use std::f64::consts::PI;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn sub(&self, ov: &GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

struct GeoS2Point {
    geo_r3_vector: Box<GeoR3Vector>,
}

struct GeoS1ChordAngle(f64);

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: GeoS1ChordAngle,
}

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(
        4.0,
        x.geo_r3_vector.sub(&y.geo_r3_vector).norm2(),
    ))
}

fn contains_point(c: &GeoS2Cap, p: &GeoS2Point) -> bool {
    geo_s2_chord_angle_between_points(&c.center, p).0 <= c.radius.0
}
