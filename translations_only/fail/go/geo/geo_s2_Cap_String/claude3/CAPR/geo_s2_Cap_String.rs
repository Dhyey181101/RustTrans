
use std::f64::consts::PI;
use std::f64;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = PI / 180.0 * GEO_S1_RADIAN;

fn geo_s2_cap_string(c: &Box<geo_s2_Cap>) -> String {
    format!(
        "[Center={:?}, Radius={:.6}]",
        c.center.geo_r3_vector, geo_s1_angle_degrees(&c.radius)
    )
}

fn geo_s2_cap_radius(c: &Box<geo_s2_Cap>) -> f64 {
    c.radius
}

fn geo_s1_chord_angle_angle(c: &geo_s1_ChordAngle) -> f64 {
    if *c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if geo_s1_chord_angle_is_infinity(c) {
        return geo_s1_inf_angle();
    }
    2.0 * (0.5 * (c.sqrt()).asin())
}

fn geo_s1_chord_angle_is_infinity(c: &geo_s1_ChordAngle) -> bool {
    f64::is_infinite(*c)
}

fn geo_s1_inf_angle() -> f64 {
    f64::INFINITY
}

fn geo_s1_angle_degrees(a: &geo_s1_Angle) -> f64 {
    *a / GEO_S1_DEGREE
}

#[derive(Debug)]
struct geo_s2_Cap {
    center: Box<geo_s2_Point>,
    radius: geo_s1_ChordAngle,
}

#[derive(Debug)]
struct geo_s2_Point {
    geo_r3_vector: Box<geo_r3_Vector>,
}

#[derive(Debug)]
struct geo_r3_Vector;

type geo_s1_ChordAngle = f64;

type geo_s1_Angle = f64;
