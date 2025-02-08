

use std::f64;

pub type geo_s1_ChordAngle = f64;
pub type geo_r3_Vector = (f64, f64, f64);
pub type geo_s2_Point = geo_r3_Vector;

pub fn geo_s2_chord_angle_between_points(x: geo_s2_Point, y: geo_s2_Point) -> geo_s1_ChordAngle {
    let v: geo_r3_Vector = sub_vector(x, y);
    f64::min(4.0, norm2(&v))
}

pub fn sub_vector(x: geo_s2_Point, y: geo_s2_Point) -> geo_r3_Vector {
    (x.0 - y.0, x.1 - y.1, x.2 - y.2)
}

pub fn dot(x: geo_r3_Vector, y: geo_r3_Vector) -> f64 {
    x.0 * y.0 + x.1 * y.1 + x.2 * y.2
}

pub fn norm2(x: &geo_r3_Vector) -> f64 {
    dot(*x, *x)
}

