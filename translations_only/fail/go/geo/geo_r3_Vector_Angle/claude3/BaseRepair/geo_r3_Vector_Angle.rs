

use std::f64::consts::PI;

const GEO_S1_RADIAN: f64 = PI / 180.0;

fn geo_r3_vector_angle(v: &Box<geo_r3_Vector>, ov: &Box<geo_r3_Vector>) -> f64 {
    let cross = geo_r3_vector_cross(Box::new(*v.clone()), ov);
    let dot = geo_r3_vector_dot(v, ov);
    let norm = geo_r3_vector_norm(&cross) / dot;
    norm.atan2(1.0) * GEO_S1_RADIAN
}

fn geo_r3_vector_cross(v: Box<geo_r3_Vector>, ov: &Box<geo_r3_Vector>) -> Box<geo_r3_Vector> {
    Box::new(geo_r3_Vector {
        x: v.y * ov.z - v.z * ov.y,
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x,
    })
}

fn geo_r3_vector_norm(v: &Box<geo_r3_Vector>) -> f64 {
    (geo_r3_vector_dot(v, v)).sqrt()
}

fn geo_r3_vector_dot(v: &Box<geo_r3_Vector>, ov: &Box<geo_r3_Vector>) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Clone)]
struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

