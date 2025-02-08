
use std::cmp::Ordering;

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: geo_s1_ChordAngle = geo_s1_ChordAngle(4.0);
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

#[derive(PartialEq, PartialOrd)]
struct geo_s1_ChordAngle(f64);

fn intersects(c: Box<geo_s2_Cap>, other: Box<geo_s2_Cap>) -> bool {
    if is_empty(&*c) || is_empty(&*other) {
        return false;
    }

    let radius_sum = add(c.radius, other.radius);
    let chord_angle = geo_s2_chord_angle_between_points(&*c.center, &*other.center);

    match radius_sum.partial_cmp(&chord_angle) {
        Some(Ordering::Greater) | Some(Ordering::Equal) => true,
        _ => false,
    }
}

fn is_empty(c: &geo_s2_Cap) -> bool {
    c.radius < geo_s1_ChordAngle(0.0)
}

fn add(c: geo_s1_ChordAngle, other: geo_s1_ChordAngle) -> geo_s1_ChordAngle {
    if other == geo_s1_ChordAngle(0.0) {
        return c;
    }

    if (c.0 + other.0) >= GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }

    let x = c.0 * (1.0 - 0.25 * other.0);
    let y = other.0 * (1.0 - 0.25 * c.0);

    geo_s1_ChordAngle(x + y + 2.0 * (x * y).sqrt().min(GEO_S1_MAX_LENGTH2))
}

fn geo_s2_chord_angle_between_points(x: &geo_s2_Point, y: &geo_s2_Point) -> geo_s1_ChordAngle {
    geo_s1_ChordAngle((vector_norm2(&sub_vectors(&*x.vec, &*y.vec))).min(4.0))
}

fn sub_vectors(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn vector_norm2(v: &geo_r3_Vector) -> f64 {
    vector_dot(v, v)  
}

fn vector_dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

struct geo_s2_Cap {
    center: Box<geo_s2_Point>,
    radius: geo_s1_ChordAngle,
}

struct geo_s2_Point {
    vec: Box<geo_r3_Vector>, 
}

struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

