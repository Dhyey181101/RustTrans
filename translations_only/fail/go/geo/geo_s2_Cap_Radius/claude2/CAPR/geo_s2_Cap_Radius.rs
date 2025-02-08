

use std::f64::{self, INFINITY};

struct GeoS2Cap {
    radius: f64,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector;

type GeoS1ChordAngle = f64;

type GeoS1Angle = f64;

const GEO_S1_RADIAN: GeoS1Angle = 1.0;

fn radius(c: &GeoS2Cap) -> GeoS1Angle {
    angle(c.radius)
}

fn angle(c: GeoS1ChordAngle) -> GeoS1Angle {
    if c < 0.0 {
        -1.0 * GEO_S1_RADIAN
    } else if is_infinity(c) {
        geo_s1_inf_angle()
    } else {
        2.0 * f64::asin(0.5 * f64::sqrt(c as f64))
    }
}

fn is_infinity(c: GeoS1ChordAngle) -> bool {
    f64::is_infinite(c)
}   

fn geo_s1_inf_angle() -> GeoS1Angle {
    INFINITY
}

