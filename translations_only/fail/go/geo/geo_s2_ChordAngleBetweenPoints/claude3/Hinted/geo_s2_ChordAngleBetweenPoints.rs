
use std::f64;

fn geo_s2_chord_angle_between_points(x: Box<geo_s2_Point>, y: Box<geo_s2_Point>) -> f64 {
    f64::min(4.0, chord_angle(&x.geo_r3_vector, &y.geo_r3_vector))
}

fn chord_angle(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    (squared_norm(&sub(v, ov))).sqrt()
}

fn sub(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn squared_norm(v: &geo_r3_Vector) -> f64 {
    dot(v, v)
}

fn dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    (v.x * ov.x) + (v.y * ov.y) + (v.z * ov.z)
}

struct geo_s2_Point {
    geo_r3_vector: geo_r3_Vector,
}

struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}
