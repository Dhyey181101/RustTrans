
use std::f64::INFINITY;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s1_inf_angle() -> f64 {
    INFINITY
}

fn geo_s1_chord_angle_angle(c: f64) -> f64 {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if c.is_infinite() {
        return geo_s1_inf_angle();
    }
    2.0 * (0.5 * c.sqrt()).asin()
}

fn geo_s1_chord_angle_is_infinity(c: f64) -> bool {
    c.is_infinite()
}

struct GeoS2Cap {
    radius: f64,
}

impl GeoS2Cap {
    fn radius(&self) -> f64 {
        geo_s1_chord_angle_angle(self.radius)
    }
}

struct GeoS2Point {
    geo_r3_vector: Box<GeoR3Vector>,
}

struct GeoR3Vector;

type GeoS1ChordAngle = f64;

type GeoS1Angle = f64;
