

use std::f64;
use std::isize;
use std::option::Option;
use std::boxed;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s1_inf_angle() -> f64 {
    f64::INFINITY
}

fn chord_angle(c: f64) -> f64 {
    if c < 0.0 {
        -1.0 * GEO_S1_RADIAN
    } else if c.is_infinite() {
        geo_s1_inf_angle()
    } else {
        2.0 * f64::asin(0.5 * f64::sqrt(c * 0.5))
    }
}

fn is_infinity(c: f64) -> bool {
    c.is_infinite()
}

type GeoS1ChordAngle = f64;
type GeoS1Angle = f64;

