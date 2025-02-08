

use std::f64;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

const EPSILON: f64 = 5e-14;

#[derive(Debug, Clone, Copy)]
struct Geo_r3_Vector {
x: f64,
y: f64,
z: f64,
}

fn is_unit_vector(v: &Geo_r3_Vector) -> bool {
let v_norm2 = (v.x * v.x + v.y * v.y + v.z * v.z).abs();
(v_norm2 - 1.0).abs() <= EPSILON
}

fn norm2(v: &Geo_r3_Vector) -> f64 {
v.x * v.x + v.y * v.y + v.z * v.z
}

fn dot(v: &Geo_r3_Vector, ov: &Geo_r3_Vector) -> f64 {
v.x * ov.x + v.y * ov.y + v.z * ov.z
}

