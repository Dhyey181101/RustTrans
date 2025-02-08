
use std::f64::consts::PI;

const GEO_S1_RADIAN: f64 = PI / 180.0;

fn geo_s1_chord_angle_angle(c: f64) -> f64 {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if geo_s1_chord_angle_is_infinity(c) {
        return geo_s1_inf_angle();
    }
    return 2.0 * (0.5 * (c.sqrt()).asin()) * GEO_S1_RADIAN;
}

fn geo_s1_chord_angle_is_infinity(c: f64) -> bool {
    return c.is_infinite();
}

fn geo_s1_inf_angle() -> f64 {
    return f64::INFINITY;
}

type GeoS1ChordAngle = f64;

type GeoS1Angle = f64;
