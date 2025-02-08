
use std::f64::{self, INFINITY};

const GEO_S1_NEGATIVE_CHORD_ANGLE: geo_s1_ChordAngle = geo_s1_ChordAngle(-1.0);

fn geo_s1_chord_angle_from_angle(a: &geo_s1_Angle) -> geo_s1_ChordAngle {
    if geo_s1_angle_is_negative(a) {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if geo_s1_angle_is_inf(a) {
        return geo_s1_inf_chord_angle();
    }
    let l = 2.0 * geo_s1_angle_sin_half_min_pi(a);
    geo_s1_ChordAngle(l * l)
}

fn geo_s1_angle_is_negative(a: &geo_s1_Angle) -> bool {
    geo_s1_angle_to_f64(a) < 0.0  
}

fn geo_s1_angle_is_inf(a: &geo_s1_Angle) -> bool {
    f64::is_infinite(geo_s1_angle_to_f64(a))
}

fn geo_s1_inf_chord_angle() -> geo_s1_ChordAngle {
    geo_s1_ChordAngle(INFINITY)  
}

fn geo_s1_angle_to_f64(a: &geo_s1_Angle) -> f64 {
    a.0
}

fn geo_s1_angle_sin_half_min_pi(a: &geo_s1_Angle) -> f64 {
    f64::sin(0.5 * f64::min(std::f64::consts::PI, geo_s1_angle_to_f64(a))) 
}

struct geo_s1_Angle(f64);

struct geo_s1_ChordAngle(f64);

