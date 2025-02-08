
use std::f64::consts::PI;
use std::f64::INFINITY;

const GEO_S1_NEGATIVE_CHORD_ANGLE: f64 = -1.0;

pub fn geo_s1_chord_angle_from_angle(a: f64) -> f64 {
    if a < 0.0 {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if a.is_infinite() {
        return geo_s1_inf_chord_angle();
    }
    let l = 2.0 * f64::sin(0.5 * f64::min(PI, a.to_radians()));
    return l * l;
}

pub fn geo_s1_inf_chord_angle() -> f64 {
    return INFINITY;
}
