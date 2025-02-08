
use std::f64::INFINITY;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s2_cap_radius(c: &Box<geo_s2_Cap>) -> f64 {
    geo_s1_chord_angle_angle(c.radius)
}

fn geo_s1_chord_angle_angle(c: geo_s1_ChordAngle) -> f64 {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if geo_s1_chord_angle_is_infinity(&c) {
        return geo_s1_inf_angle();
    }
    return 2.0 * (0.5 * c.sqrt()).asin();
}

fn geo_s1_chord_angle_is_infinity(c: &geo_s1_ChordAngle) -> bool {
    c.is_infinite()
}

fn geo_s1_inf_angle() -> f64 {
    INFINITY
}

struct geo_s2_Cap {
    radius: geo_s1_ChordAngle,
}

struct geo_s2_Point {
    geo_r3_vector: Box<geo_r3_Vector>,
}

struct geo_r3_Vector;

type geo_s1_ChordAngle = f64;

type geo_s1_Angle = f64;
