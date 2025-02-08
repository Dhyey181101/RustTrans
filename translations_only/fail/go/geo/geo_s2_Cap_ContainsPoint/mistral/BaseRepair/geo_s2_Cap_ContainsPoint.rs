

use std::f64;

pub struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64,
}

pub struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

pub struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

pub fn geo_s2_contains_point(c: &GeoS2Cap, p: &GeoS2Point) -> bool {
    let chord_angle = geo_s2_chord_angle_between_points(&c.center.geo_r3_vector, &p.geo_r3_vector);
    chord_angle.0 <= c.radius
}

pub fn geo_s2_chord_angle_between_points(x: &GeoR3Vector, y: &GeoR3Vector) -> GeoS1ChordAngle {
    let dist_sq: f64 = x.sub(y).norm_sq();
    GeoS1ChordAngle(dist_sq.min(4.0))
}

impl GeoR3Vector {
    pub fn sub(&self, ov: &GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    pub fn norm_sq(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

#[derive(PartialEq)]
pub struct GeoS1ChordAngle(f64);

pub fn geo_s1_chord_angle_less_than(a: &GeoS1ChordAngle, b: &GeoS1ChordAngle) -> bool {
    a.0 < b.0
}

