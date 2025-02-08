
use std::f64::consts::PI;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s1_chord_angle_angle(c: geo_s1_ChordAngle) -> geo_s1_Angle {
    if c < 0.0 {
        return -GEO_S1_RADIAN;
    }
    if geo_s1_chord_angle_is_infinity(c) {
        return geo_s1_inf_angle();
    }
    return 2.0 * (0.5 * (c.sqrt()).asin());
}

fn geo_s1_chord_angle_is_infinity(c: geo_s1_ChordAngle) -> bool {
    c.is_infinite()
}

fn geo_s1_inf_angle() -> geo_s1_Angle {
    f64::INFINITY
}

type geo_s1_ChordAngle = f64;

type geo_s1_Angle = f64;
