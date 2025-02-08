
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

pub fn geo_s2_cap_intersects(c: &geo_s2_cap, other: &geo_s2_cap) -> bool {
    if c.radius < 0.0 || other.radius < 0.0 {
        return false;
    }

    c.radius + other.radius >= geo_s2_chord_angle_between_points(&c.center, &other.center)
}

pub fn geo_s1_chord_angle_add(c: f64, other: f64) -> f64 {
    // Note that this method (and Sub) is much more efficient than converting
    // the ChordAngle to an Angle and adding those and converting back. It
    // requires only one square root plus a few additions and multiplications.

    // Optimization for the common case where b is an error tolerance
    // parameter that happens to be set to zero.
    if other == 0.0 {
        return c;
    }

    // Clamp the angle sum to at most 180 degrees.
    if c + other >= GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }

    // Let a and b be the (non-squared) chord lengths, and let c = a+b.
    // Let A, B, and C be the corresponding half-angles (a = 2*sin(A), etc).
    // Then the formula below can be derived from c = 2 * sin(A+B) and the
    // relationships   sin(A+B) = sin(A)*cos(B) + sin(B)*cos(A)
    //                 cos(X) = sqrt(1 - sin^2(X))
    let x = c * (1.0 - 0.25 * other);
    let y = other * (1.0 - 0.25 * c);
    f64::min(GEO_S1_MAX_LENGTH2, x + y + 2.0 * f64::sqrt(x * y))
}

pub fn geo_s2_chord_angle_between_points(x: &geo_s2_point, y: &geo_s2_point) -> f64 {
    f64::min(4.0, geo_r3_vector_norm2(&geo_r3_vector_sub(&x.geo_r3_vector, &y.geo_r3_vector)))
}

pub fn geo_r3_vector_sub(v: &geo_r3_vector, ov: &geo_r3_vector) -> geo_r3_vector {
    geo_r3_vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

pub fn geo_r3_vector_norm2(v: &geo_r3_vector) -> f64 {
    geo_r3_vector_dot(v, v)
}

pub fn geo_r3_vector_dot(v: &geo_r3_vector, ov: &geo_r3_vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Copy, Clone)]
pub struct geo_s2_cap {
    pub center: geo_s2_point,
    pub radius: f64,
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
