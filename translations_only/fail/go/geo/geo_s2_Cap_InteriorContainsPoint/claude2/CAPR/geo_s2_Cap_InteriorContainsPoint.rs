

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: geo_s1_ChordAngle = geo_s1_ChordAngle(4.0);

#[derive(PartialEq, PartialOrd)]
struct geo_s1_ChordAngle(f64);

fn geo_s1_chord_angle_min(x: f64, y: f64) -> geo_s1_ChordAngle {
    geo_s1_ChordAngle(x.min(y))
}

fn interior_contains_point(c: Box<geo_s2_Cap>, p: geo_s2_Point) -> bool {
    is_full(c.as_ref()) || geo_s2_chord_angle_between_points(c.center, p) < c.radius
}

fn is_full(c: &geo_s2_Cap) -> bool {
    c.radius == GEO_S1_STRAIGHT_CHORD_ANGLE
}

fn geo_s2_chord_angle_between_points(x: geo_s2_Point, y: geo_s2_Point) -> geo_s1_ChordAngle {
    geo_s1_chord_angle_min(4.0, geo_r3_vector_norm2(&geo_r3_vector_sub(&x.v, &y.v)))
}

fn geo_r3_vector_sub(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z
    }
}

fn geo_r3_vector_norm2(v: &geo_r3_Vector) -> f64 {
    geo_r3_vector_dot(v, v)  
}

fn geo_r3_vector_dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

struct geo_s2_Cap {
    center: geo_s2_Point,
    radius: geo_s1_ChordAngle
}

struct geo_s2_Point {
    v: geo_r3_Vector
}

struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64  
}

