

use std::f64;

const GEO_S1_NEGATIVE_CHORD_ANGLE: f64 = -1.0;

fn geo_s1_chord_angle_from_angle(a: f64) -> f64 {
    if a < 0.0 {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if a.is_sign_positive() && a.is_infinite() {
        return f64::INFINITY;
    }
    let l = 2.0 * a.sin() / 2.0;
    return l * l;
}

struct GeoS1Angle(f64);

impl GeoS1Angle {
    fn is_inf(&self) -> bool {
        self.0.is_infinite()
    }
}

struct GeoS1ChordAngle(f64);

fn geo_s1_inf_chord_angle() -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::INFINITY)
}

fn main() {}

