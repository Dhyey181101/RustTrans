

use std::fmt;
use std::ops::Add;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

#[derive(Copy, Clone, PartialEq)]
struct GeoS1ChordAngle(f64);

impl Add for GeoS1ChordAngle {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let c = self.0;
        let other = other.0;

        if c + other >= GEO_S1_MAX_LENGTH2 {
            return GeoS1ChordAngle(GEO_S1_STRAIGHT_CHORD_ANGLE);
        }

        let x = c * (1.0 - 0.25 * other);
        let y = other * (1.0 - 0.25 * c);

        GeoS1ChordAngle(f64::min(GEO_S1_MAX_LENGTH2, x + y + 2.0 * (x * y).sqrt()))
    }
}

impl fmt::Debug for GeoS1ChordAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeoS1ChordAngle({})", self.0)
    }
}

#[derive(Debug)]
struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

#[derive(Debug)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn intersects(&self, other: &GeoS2Cap) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        self.radius.0
            + other
                .radius
                .0
                >=
            self.chord_angle_between_points(&other.center).0
    }

    fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }

    fn chord_angle_between_points(&self, other: &GeoS2Point) -> GeoS1ChordAngle {
        GeoS1ChordAngle(f64::min(GEO_S1_MAX_LENGTH2, self.center.geo_r3_vector.distance_squared_to(&other.geo_r3_vector)))
    }
}

#[derive(Debug)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn sub(&self, other: &GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn distance_squared_to(&self, other: &GeoR3Vector) -> f64 {
        self.dot(self.sub(other))
    }

    fn dot(&self, other: GeoR3Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

