
use std::f64::consts::PI;

const GEO_S1_RADIAN: GeoS1Angle = 1.0;

struct GeoS2Cap {
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    vector: GeoR3Vector,
}

struct GeoR3Vector {}

type GeoS1ChordAngle = f64;
type GeoS1Angle = f64;

fn radius(c: &GeoS2Cap) -> GeoS1Angle {
    angle(c.radius)
}

fn angle(c: GeoS1ChordAngle) -> GeoS1Angle {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if is_infinity(c) {
        return geo_s1_inf_angle();
    }
    2.0 * (0.5 * (c as f64).sqrt()).asin()
}

fn is_infinity(c: GeoS1ChordAngle) -> bool {
    c.is_infinite()
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    f64::INFINITY
}

fn main() {}
