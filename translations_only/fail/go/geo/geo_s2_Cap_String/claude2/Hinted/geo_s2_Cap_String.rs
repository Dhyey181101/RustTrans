
use std::f64::{self, INFINITY};
use std::fmt;
use std::ops::Mul;

#[derive(Debug)]
struct GeoS2Point {
    vector: GeoR3Vector,
}

#[derive(Debug)]
struct GeoR3Vector;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (std::f64::consts::PI / 180.0) * GEO_S1_RADIAN;

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: f64,
}

fn fmt_geo_s2_cap(cap: &GeoS2Cap, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[Center={:?}, Radius={}]", *cap.center, to_degrees(radius(cap)))
}

fn radius(cap: &GeoS2Cap) -> f64 {
    angle(cap.radius)
}

fn angle(chord_angle: f64) -> f64 {
    if chord_angle < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if is_infinite(chord_angle) {
        return geo_s1_inf_angle();
    }
    2.0 * chord_angle.sin().acos()
}

fn is_infinite(chord_angle: f64) -> bool {
    chord_angle.is_infinite()
}

fn geo_s1_inf_angle() -> f64 {
    INFINITY
}

fn to_degrees(angle: f64) -> f64 {
    angle / GEO_S1_DEGREE
}

