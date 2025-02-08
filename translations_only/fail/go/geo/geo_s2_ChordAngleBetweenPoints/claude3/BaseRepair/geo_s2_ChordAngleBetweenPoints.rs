
use std::f64;

fn geo_s2_ChordAngleBetweenPoints(x: Box<geo_s2_Point>, y: Box<geo_s2_Point>) -> f64 {
    f64::min(4.0, geo_r3_Vector_Norm2(&geo_r3_Vector_Sub(&x.geo_r3_Vector, &y.geo_r3_Vector)))
}

fn geo_r3_Vector_Sub(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> Box<geo_r3_Vector> {
    Box::new(geo_r3_Vector {
        X: v.X - ov.X,
        Y: v.Y - ov.Y,
        Z: v.Z - ov.Z,
    })
}

fn geo_r3_Vector_Norm2(v: &Box<geo_r3_Vector>) -> f64 {
    geo_r3_Vector_Dot(&v, &v)
}

fn geo_r3_Vector_Dot(v: &Box<geo_r3_Vector>, ov: &Box<geo_r3_Vector>) -> f64 {
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

