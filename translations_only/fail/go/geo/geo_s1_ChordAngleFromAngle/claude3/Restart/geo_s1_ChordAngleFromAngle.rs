

use std::f64::consts::PI;
use std::f64::INFINITY;

type GeoS1Angle = f64;
type GeoS1ChordAngle = f64;

const GEO_S1_NEGATIVE_CHORD_ANGLE: GeoS1ChordAngle = -1.0;

fn geo_s1_chord_angle_from_angle(a: GeoS1Angle) -> GeoS1ChordAngle {
    if a < 0.0 {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if a.is_infinite() {
        return geo_s1_inf_chord_angle();
    }
    let l = 2.0 * (0.5 * a.min(PI)).sin();
    GeoS1ChordAngle::powi(l, 2)
}

fn geo_s1_angle_is_inf(a: GeoS1Angle) -> bool {
    a.is_infinite()
}

fn geo_s1_inf_chord_angle() -> GeoS1ChordAngle {
    INFINITY
}

fn geo_s1_angle_radians(a: GeoS1Angle) -> f64 {
    a
}

fn geo_s1_chord_angle(a: f64) -> GeoS1ChordAngle {
    a
}

