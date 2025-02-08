
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: GeoS1ChordAngle = GeoS1ChordAngle(4.0);
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn geo_s2_cap_from_center_height(center: GeoS2Point, height: f64) -> GeoS2Cap {
    geo_s2_cap_from_center_chord_angle(center, geo_s1_chord_angle_from_squared_length(2.0 * height))
}

fn geo_s1_chord_angle_from_squared_length(length2: f64) -> GeoS1ChordAngle {
    if length2 > GEO_S1_MAX_LENGTH2 {
        GEO_S1_STRAIGHT_CHORD_ANGLE
    } else {
        GeoS1ChordAngle(length2)
    }
}

fn geo_s2_cap_from_center_chord_angle(center: GeoS2Point, radius: GeoS1ChordAngle) -> GeoS2Cap {
    GeoS2Cap {
        center: center,
        radius: radius,
    }
}

#[derive(Copy, Clone)]
struct GeoS1Angle(f64);

#[derive(Copy, Clone)] 
struct GeoS1ChordAngle(f64);

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector;
