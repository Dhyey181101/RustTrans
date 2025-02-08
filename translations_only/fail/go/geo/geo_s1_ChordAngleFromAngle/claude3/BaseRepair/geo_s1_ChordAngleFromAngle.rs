
use std::f64::consts::PI;

const GEO_S1_NEGATIVE_CHORD_ANGLE: f64 = -1.0 * -1.0;

fn geo_s1_chord_angle_from_angle(a: f64) -> f64 {
    if a < 0.0 {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if a.is_infinite() {
        return std::f64::INFINITY;
    }
    let l = 2.0 * (0.5 * a.min(PI)).sin();
    l * l
}

fn is_inf(a: f64) -> bool {
    a.is_infinite()
}

fn radians(a: f64) -> f64 {
    a
}

type GeoS1Angle = f64;

type GeoS1ChordAngle = f64;
