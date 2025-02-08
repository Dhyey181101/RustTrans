
use std::f64;

const GEO_S1_NEGATIVE_CHORD_ANGLE: f64 = -1.0;

fn geo_s1_chord_angle_from_angle(a: f64) -> f64 {
    if a < 0.0 {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if a.is_sign_positive() && a.is_infinite() {
        return f64::INFINITY;
    }
    let l = 2.0 * a.sin().abs().sqrt();
    l * l
}

fn is_inf(f: f64) -> bool {
    f.is_sign_positive() && f.is_infinite()
}

fn geo_s1_inf_chord_angle() -> f64 {
    f64::INFINITY
}

fn radians(f: f64) -> f64 {
    f
}

#[derive(Copy, Clone, Debug)]
struct GeoS1Angle(f64);

#[derive(Copy, Clone, Debug)]
struct GeoS1ChordAngle(f64);
