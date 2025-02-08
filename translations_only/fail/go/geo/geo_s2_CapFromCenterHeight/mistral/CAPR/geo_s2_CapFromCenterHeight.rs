
use std::boxed::Box;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

type GeoS1Angle = f64;
type GeoS1ChordAngle = f64;
type GeoS2Cap = (GeoS2Point, GeoS1ChordAngle);
type GeoS2Point = (Box<GeoR3Vector>, f64, f64);
type GeoR3Vector = ();

fn geo_s2_cap_from_center_height(
    center: GeoS2Point,
    height: f64,
) -> GeoS2Cap {
    let chord_angle = geo_s1_chord_angle_from_squared_length(2.0 * height);
    (center, chord_angle)
}

fn geo_s1_chord_angle_from_squared_length(length2: f64) -> GeoS1ChordAngle {
    if length2 > GEO_S1_MAX_LENGTH2 {
        GEO_S1_STRAIGHT_CHORD_ANGLE
    } else {
        length2
    }
}

fn geo_s2_cap_from_center_chord_angle(
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
) -> GeoS2Cap {
    (center, radius)
}
