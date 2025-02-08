
use std::f64;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s1_chord_angle_angle(c: f64) -> f64 {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if geo_s1_chord_angle_is_infinity(c) {
        return geo_s1_inf_angle();
    }
    return 2.0 * f64::asin(0.5 * f64::sqrt(c));
}

fn geo_s1_chord_angle_is_infinity(c: f64) -> bool {
    return f64::is_infinite(c);
}

fn geo_s1_inf_angle() -> f64 {
    return f64::INFINITY;
}

type GeoS1ChordAngle = f64;

type GeoS1Angle = f64;
