
use std::f64::consts::PI;

pub fn geo_s2_chord_angle_between_points(x: geo_s2_point, y: geo_s2_point) -> geo_s1_chord_angle {
    geo_s1_chord_angle(f64::min(4.0, (x.geo_r3_vector.x - y.geo_r3_vector.x).powf(2.0) + (x.geo_r3_vector.y - y.geo_r3_vector.y).powf(2.0) + (x.geo_r3_vector.z - y.geo_r3_vector.z).powf(2.0)))
}

pub struct geo_r3_vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct geo_s2_point {
    pub geo_r3_vector: geo_r3_vector,
}

pub struct geo_s1_chord_angle(pub f64);
