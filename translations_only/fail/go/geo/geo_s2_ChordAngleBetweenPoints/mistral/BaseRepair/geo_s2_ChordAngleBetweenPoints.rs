
use std::f64;

pub type geo_s1_ChordAngle = f64;
pub type geo_r3_Vector = (f64, f64, f64);
pub type geo_s2_Point = geo_r3_Vector;

pub fn geo_s2_ChordAngleBetweenPoints(x: geo_s2_Point, y: geo_s2_Point) -> geo_s1_ChordAngle {
    geo_s1_ChordAngle::from(f64::min(4.0, norm2(sub(x, y))))
}

pub fn sub(a: geo_r3_Vector, b: geo_r3_Vector) -> geo_r3_Vector {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

pub fn norm2(v: geo_r3_Vector) -> f64 {
    dot(v, v)
}

pub fn dot(a: geo_r3_Vector, b: geo_r3_Vector) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}
