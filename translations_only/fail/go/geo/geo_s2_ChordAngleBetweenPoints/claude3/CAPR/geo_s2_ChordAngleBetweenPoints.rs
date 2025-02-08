
use std::f64;

fn geo_s2_ChordAngleBetweenPoints(x: Box<geo_s2_Point>, y: Box<geo_s2_Point>) -> f64 {
    f64::min(4.0, vector_norm2(&vector_sub(&x.geo_r3_Vector, &y.geo_r3_Vector)))
}

fn vector_sub(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> geo_r3_Vector {
    geo_r3_Vector {
        X: v.X - ov.X,
        Y: v.Y - ov.Y,
        Z: v.Z - ov.Z,
    }
}

fn vector_norm2(v: &geo_r3_Vector) -> f64 {
    vector_dot(v, v)
}

fn vector_dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    (v.X * ov.X) as f64 + (v.Y * ov.Y) as f64 + (v.Z * ov.Z) as f64
}

struct geo_s2_Point {
    geo_r3_Vector: geo_r3_Vector,
}

struct geo_r3_Vector {
    X: f64,
    Y: f64,
    Z: f64,
}
