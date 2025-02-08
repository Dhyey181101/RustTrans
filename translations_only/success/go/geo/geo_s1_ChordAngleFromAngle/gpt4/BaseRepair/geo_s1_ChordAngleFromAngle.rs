
use std::f64::consts::PI;

const GEO_S1_NEGATIVE_CHORD_ANGLE: GeoS1ChordAngle = GeoS1ChordAngle(-1.0);

fn geo_s1_chord_angle_from_angle(a: GeoS1Angle) -> GeoS1ChordAngle {
    if a.0 < 0.0 {
        return GEO_S1_NEGATIVE_CHORD_ANGLE;
    }
    if a.is_inf() {
        return geo_s1_inf_chord_angle();
    }
    let l = 2.0 * (0.5 * a.radians()).min(PI / 2.0).sin();
    GeoS1ChordAngle(l * l)
}

fn geo_s1_inf_chord_angle() -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::INFINITY)
}

#[derive(Copy, Clone)]
struct GeoS1Angle(f64);

impl GeoS1Angle {
    fn is_inf(self) -> bool {
        self.0.is_infinite()
    }

    fn radians(self) -> f64 {
        self.0
    }
}

#[derive(Copy, Clone)]
struct GeoS1ChordAngle(f64);
