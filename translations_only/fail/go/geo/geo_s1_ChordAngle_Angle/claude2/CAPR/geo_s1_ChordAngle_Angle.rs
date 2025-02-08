
use std::f64::INFINITY;

struct GeoS1Angle(f64);

struct GeoS1ChordAngle(f64);

fn geo_s1_radian() -> GeoS1Angle {
    GeoS1Angle(1.0)
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    GeoS1Angle(INFINITY)
}

fn negate(angle: GeoS1Angle) -> GeoS1Angle {
    GeoS1Angle(-angle.0)
}

fn angle(chord_angle: &GeoS1ChordAngle) -> GeoS1Angle {
    if chord_angle.0 < 0.0 {
        return negate(geo_s1_radian());
    }
    if is_infinity(chord_angle) {
        return geo_s1_inf_angle();
    }
    GeoS1Angle(2.0 * (chord_angle.0).sin().acos() * 0.5)  
}

fn is_infinity(chord_angle: &GeoS1ChordAngle) -> bool {
    chord_angle.0.is_infinite()
}

