

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = PI;

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn interior_contains_point(&self, p: &GeoS2Point) -> bool {
        self.is_full() || geo_s2_chord_angle_between_points(&self.center, p) < self.radius
    }

    fn is_full(&self) -> bool {
        self.radius == GEO_S1_STRAIGHT_CHORD_ANGLE
    }
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

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

type GeoS1ChordAngle = f64;

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    let diff = x.geo_r3_vector.sub(&y.geo_r3_vector);
    GeoS1ChordAngle::min(GEO_S1_STRAIGHT_CHORD_ANGLE, diff.norm2())
}

