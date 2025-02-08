

use std::f64::consts::PI;
use std::fmt;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = PI / 180.0 * GEO_S1_RADIAN;

fn geo_s2_cap_string(c: &geo_s2_Cap) -> String {
    format!(
        "[Center={:?}, Radius={:.6}]",
        c.center.geo_r3_vector_box, geo_s1_angle_degrees(&geo_s2_cap_radius(c))
    )
}

fn geo_s2_cap_radius(c: &geo_s2_Cap) -> geo_s1_Angle {
    geo_s1_chord_angle_angle(&c.radius)
}

fn geo_s1_chord_angle_angle(c: &geo_s1_ChordAngle) -> geo_s1_Angle {
    if *c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if geo_s1_chord_angle_is_infinity(c) {
        return geo_s1_inf_angle();
    }
    2.0 * (0.5 * (c.sqrt()).asin()) as geo_s1_Angle
}

fn geo_s1_chord_angle_is_infinity(c: &geo_s1_ChordAngle) -> bool {
    c.is_infinite()
}

fn geo_s1_inf_angle() -> geo_s1_Angle {
    std::f64::INFINITY as geo_s1_Angle
}

fn geo_s1_angle_degrees(a: &geo_s1_Angle) -> f64 {
    (*a / GEO_S1_DEGREE) as f64
}

struct geo_s2_Cap {
    center: geo_s2_Point,
    radius: geo_s1_ChordAngle,
}

struct geo_s2_Point {
    geo_r3_vector_box: Box<geo_r3_Vector>,
}

#[derive(Debug)]
struct geo_r3_Vector;

type geo_s1_ChordAngle = f64;

type geo_s1_Angle = f64;

