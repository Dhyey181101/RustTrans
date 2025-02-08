
use std::f64::{self, INFINITY};

const GEO_S1_RADIAN: f64 = 1.0;

struct GeoS1ChordAngle(f64);

struct GeoS1Angle(f64);

fn angle(c: &GeoS1ChordAngle) -> GeoS1Angle {
    if c.0 < 0.0 {
        return GeoS1Angle(-1.0 * GEO_S1_RADIAN);
    }
    if is_infinity(c) {
        return inf_angle();
    }
    let x = 0.5 * f64::sqrt(c.0);
    GeoS1Angle(2.0 * x.asin())
}

fn is_infinity(c: &GeoS1ChordAngle) -> bool {
    c.0.is_infinite()
}   

fn inf_angle() -> GeoS1Angle {
    GeoS1Angle(INFINITY)
}

