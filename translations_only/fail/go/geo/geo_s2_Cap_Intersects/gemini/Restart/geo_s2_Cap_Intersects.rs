
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

pub fn geo_s2_cap_intersects(c: &geo_s2_cap, other: &geo_s2_cap) -> bool {
    if c.radius + other.radius < geo_s2_chord_angle_between_points(&c.center, &other.center) {
        return false;
    }
    true
}

pub fn geo_s2_cap_is_empty(c: &geo_s2_cap) -> bool {
    c.radius < 0.0
}

pub fn geo_s1_chord_angle_add(c: f64, other: f64) -> f64 {
    if other == 0.0 {
        return c;
    }
    if c + other >= GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }
    let x = (c * (1.0 - 0.25 * other)).min(GEO_S1_MAX_LENGTH2);
    let y = (other * (1.0 - 0.25 * c)).min(GEO_S1_MAX_LENGTH2);
    x + y + 2.0 * (x * y).sqrt()
}

pub fn geo_s2_chord_angle_between_points(x: &geo_s2_point, y: &geo_s2_point) -> f64 {
    geo_s1_chord_angle_add(geo_r3_vector_norm2(&geo_r3_vector_sub(&x.geo_r3_vector, &y.geo_r3_vector)), 0.0).min(4.0)
}

pub fn geo_r3_vector_sub(v: &geo_r3_vector, ov: &geo_r3_vector) -> geo_r3_vector {
    geo_r3_vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

pub fn geo_r3_vector_norm2(v: &geo_r3_vector) -> f64 {
    v.x * v.x + v.y * v.y + v.z * v.z
}

pub fn geo_r3_vector_dot(v: &geo_r3_vector, ov: &geo_r3_vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

pub struct geo_s2_cap {
    pub center: geo_s2_point,
    pub radius: f64,
}

pub struct geo_s2_point {
    pub geo_r3_vector: geo_r3_vector,
}

pub struct geo_r3_vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
