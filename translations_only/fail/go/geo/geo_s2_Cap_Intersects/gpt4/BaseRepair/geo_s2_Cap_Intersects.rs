
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: GeoS1ChordAngle = GeoS1ChordAngle(4.0);
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct GeoS1ChordAngle(f64);

#[derive(Clone, Copy)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn intersects(&self, other: &GeoS2Cap) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        self.radius.add(other.radius) >= geo_s2_chord_angle_between_points(self.center, other.center)
    }

    fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }
}

impl GeoS1ChordAngle {
    fn add(self, other: GeoS1ChordAngle) -> GeoS1ChordAngle {
        if other.0 == 0.0 {
            return self;
        }

        if self.0 + other.0 >= GEO_S1_MAX_LENGTH2 {
            return GEO_S1_STRAIGHT_CHORD_ANGLE;
        }

        let x = self.0 * (1.0 - 0.25 * other.0);
        let y = other.0 * (1.0 - 0.25 * self.0);
        GeoS1ChordAngle(f64::min(GEO_S1_MAX_LENGTH2, x + y + 2.0 * f64::sqrt(x * y)))
    }
}

#[derive(Clone, Copy)]
struct GeoS2Point(GeoR3Vector);

fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, x.0.sub(y.0).norm2()))
}

#[derive(Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn sub(self, ov: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    fn norm2(self) -> f64 {
        self.dot(self)
    }

    fn dot(self, ov: GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}
