
use std::f64::{self, INFINITY};
use std::fmt;
use std::ops::Mul;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (std::f64::consts::PI / 180.0) * GEO_S1_RADIAN;

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: f64,
}

#[derive(Debug)]
struct GeoS2Point {
    vector: GeoR3Vector,
}

#[derive(Debug)]
struct GeoR3Vector;

impl fmt::Display for GeoS2Cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Center={:?}, Radius={}]", *self.center, to_degrees(self.radius))
    }
}

fn to_degrees(angle: f64) -> f64 {
    angle / GEO_S1_DEGREE
}

fn radius(cap: &GeoS2Cap) -> f64 {
    cap.radius
}

fn angle(chord_angle: f64) -> f64 {
    if chord_angle < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if is_infinity(chord_angle) {
        return f64::INFINITY;
    }
    2.0 * chord_angle.sin().asin()
}

fn is_infinity(chord_angle: f64) -> bool {
    chord_angle.is_infinite()
}

