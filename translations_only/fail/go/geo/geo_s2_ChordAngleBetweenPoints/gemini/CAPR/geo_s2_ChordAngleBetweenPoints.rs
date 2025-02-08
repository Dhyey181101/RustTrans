
use std::f64::consts::PI;

pub fn geo_s2_chord_angle_between_points(x: geo_s2_point, y: geo_s2_point) -> geo_s1_chord_angle {
    geo_s1_chord_angle(f64::min(4.0, norm2(sub(x.geo_r3_vector, y.geo_r3_vector))))
}

pub fn sub(v: geo_r3_vector, ov: geo_r3_vector) -> geo_r3_vector {
    geo_r3_vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

pub fn norm2(v: geo_r3_vector) -> f64 {
    dot(v, v)
}

pub fn dot(v: geo_r3_vector, ov: geo_r3_vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Copy, Clone)]
pub struct geo_s2_point {
    pub geo_r3_vector: geo_r3_vector,
}

#[derive(Copy, Clone)]
pub struct geo_r3_vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone)]
pub struct geo_s1_chord_angle(pub f64);

