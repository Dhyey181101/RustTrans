

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = PI;

fn geo_s2_cap_interior_contains_point(cap: &Box<geo_s2_Cap>, p: &geo_s2_Point) -> bool {
    return geo_s2_cap_is_full(cap) || geo_s2_chord_angle_between_points(&cap.center, p) < cap.radius;
}

fn geo_s2_cap_is_full(cap: &Box<geo_s2_Cap>) -> bool {
    return cap.radius == GEO_S1_STRAIGHT_CHORD_ANGLE;
}

fn geo_s2_chord_angle_between_points(x: &geo_s2_Point, y: &geo_s2_Point) -> f64 {
    let diff = geo_r3_vector_sub(&x.geo_r3_vector, &y.geo_r3_vector);
    return f64::min(GEO_S1_STRAIGHT_CHORD_ANGLE, geo_r3_vector_norm2(&diff));
}

fn geo_r3_vector_sub(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> geo_r3_Vector {
    return geo_r3_Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    };
}

fn geo_r3_vector_norm2(v: &geo_r3_Vector) -> f64 {
    return geo_r3_vector_dot(v, v);
}

fn geo_r3_vector_dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    return (v.x * ov.x) + (v.y * ov.y) + (v.z * ov.z);
}

struct geo_s2_Cap {
    center: geo_s2_Point,
    radius: f64,
}

struct geo_s2_Point {
    geo_r3_vector: geo_r3_Vector,
}

struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

