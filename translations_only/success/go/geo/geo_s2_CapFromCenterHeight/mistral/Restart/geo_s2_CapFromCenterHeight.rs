
use std::boxed::Box;

pub struct GeoS2Point(());
pub struct GeoR3Vector(());
pub struct GeoS1Angle(f64);
pub struct GeoS1ChordAngle(f64);
pub struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

const GEO_S1_STRAIGHT_CHORD_ANGLE: GeoS1ChordAngle = GeoS1ChordAngle(4.0);
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

pub fn geo_s2_cap_from_center_height(
    center: GeoS2Point,
    height: f64,
) -> GeoS2Cap {
    geo_s2_cap_from_center_chord_angle(center, geo_s1_chord_angle_from_squared_length(2.0 * height))
}

pub fn geo_s1_chord_angle_from_squared_length(length2: f64) -> GeoS1ChordAngle {
    if length2 > GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }
    GeoS1ChordAngle(length2)
}

pub fn geo_s2_cap_from_center_chord_angle(
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
) -> GeoS2Cap {
    GeoS2Cap { center, radius }
}
