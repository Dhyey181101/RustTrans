

use std::f64::consts::PI;

fn geo_s2_cap_contains_point(c: &geo_s2_Cap, p: &geo_s2_Point) -> bool {
    geo_s2_chord_angle_between_points(&c.center, p) <= c.radius
}

fn geo_s2_chord_angle_between_points(x: &geo_s2_Point, y: &geo_s2_Point) -> geo_s1_ChordAngle {
    let diff = geo_r3_vector_sub(&x.geo_r3_vector, &y.geo_r3_vector);
    geo_r3_vector_norm2(&*diff)
}

fn geo_r3_vector_sub(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> Box<geo_r3_Vector> {
    Box::new(geo_r3_Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    })
}

fn geo_r3_vector_norm2(v: &geo_r3_Vector) -> f64 {
    geo_r3_vector_dot(v, v)
}

fn geo_r3_vector_dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    (v.x * ov.x) + (v.y * ov.y) + (v.z * ov.z)
}

struct geo_s2_Cap {
    center: geo_s2_Point,
    radius: geo_s1_ChordAngle,
}

struct geo_s2_Point {
    geo_r3_vector: geo_r3_Vector,
}

struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

type geo_s1_ChordAngle = f64;

