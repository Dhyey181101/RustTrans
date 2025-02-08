
use std::f64;
use std::isize;

const GEO_S1_NEGATIVE_CHORD_ANGLE: f64 = -1.0;

fn geo_s1_chord_angle_from_angle(a: f64) -> f64 {
    if a < 0.0 {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if a.is_infinite() {
        return f64::INFINITY;
    }
    let l = 2.0 * a.sin().abs().sqrt();
    return l * l;
}

fn main() {}

struct GeoS1Angle(f64);

impl GeoS1Angle {
    fn is_inf(&self) -> bool {
        self.0.is_infinite()
    }

    fn radians(&self) -> f64 {
        self.0
    }
}

struct GeoS1ChordAngle(f64);
