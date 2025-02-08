

use std::fmt;
use std::ops::Add;

#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn dot(&self, ov: GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }

    fn sub(&self, ov: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    fn norm2(&self) -> f64 {
        self.dot(*self)
    }
}

impl Add for GeoR3Vector {
    type Output = GeoR3Vector;

    fn add(self, rhs: GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl fmt::Debug for GeoR3Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeoR3Vector({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone, Default)]
struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

#[derive(Copy, Clone, Default, PartialEq)]
struct GeoS1ChordAngle(f64);

impl PartialOrd for GeoS1ChordAngle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.partial_cmp(&other.0)?)
    }
}

impl GeoS2Point {
    fn sub(&self, ov: GeoS2Point) -> GeoR3Vector {
        self.geo_r3_vector.sub(ov.geo_r3_vector)
    }
}

impl GeoS1ChordAngle {
    fn min(self, other: Self) -> Self {
        GeoS1ChordAngle(self.0.min(other.0))
    }
}

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn add_point(&mut self, p: GeoS2Point) {
        if self.is_empty() {
            self.center = p;
            self.radius = GeoS1ChordAngle(0.0);
            return;
        }

        let new_rad = geo_s2_chord_angle_between_points(self.center, p);
        self.radius = self.radius.min(new_rad);
    }

    fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }
}

fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> GeoS1ChordAngle {
    let dist = (x.sub(y)).norm2().min(4.0);
    GeoS1ChordAngle(dist)
}

fn main() {}

