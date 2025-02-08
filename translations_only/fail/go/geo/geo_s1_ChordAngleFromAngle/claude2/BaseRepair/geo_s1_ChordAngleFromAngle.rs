
use std::f64::{self, INFINITY};

const GEO_S1_NEGATIVE_CHORD_ANGLE: geo_s1_ChordAngle = geo_s1_ChordAngle(-1.0);

fn geo_s1_chord_angle_from_angle(a: f64) -> geo_s1_ChordAngle {
    if a < 0.0 {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if f64::is_infinite(a) {
        return geo_s1_inf_chord_angle();
    }
    let l = 2.0 * f64::sin(0.5 * f64::min(std::f64::consts::PI, a));
    geo_s1_ChordAngle(l * l)
}

fn geo_s1_inf_chord_angle() -> geo_s1_ChordAngle {
    geo_s1_ChordAngle(INFINITY)  
}

fn sin_half_min_pi(a: f64) -> f64 {
    f64::sin(0.5 * f64::min(std::f64::consts::PI, a))
}

struct geo_s1_ChordAngle(f64);

