
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn geo_s2_cap_intersects(c: &geo_s2_Cap, other: &geo_s2_Cap) -> bool {
    if geo_s2_cap_is_empty(c) || geo_s2_cap_is_empty(other) {
        return false;
    }

    c.radius + other.radius >= geo_s2_chord_angle_between_points(&c.center, &other.center)
}

fn geo_s2_cap_is_empty(c: &geo_s2_Cap) -> bool {
    c.radius < 0.0
}

fn geo_s1_chord_angle_add(c: f64, other: f64) -> f64 {
    if other == 0.0 {
        return c;
    }

    if c + other >= GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }

    let x = c * (1.0 - 0.25 * other);
    let y = other * (1.0 - 0.25 * c);
    (x + y + 2.0 * (x * y).sqrt()).min(GEO_S1_MAX_LENGTH2)
}

fn geo_s2_chord_angle_between_points(x: &geo_s2_Point, y: &geo_s2_Point) -> f64 {
    let diff = geo_r3_vector_sub(&x.geo_r3_vector, &y.geo_r3_vector);
    geo_r3_vector_norm2(&diff).min(GEO_S1_MAX_LENGTH2)
}

fn geo_r3_vector_sub(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn geo_r3_vector_norm2(v: &geo_r3_Vector) -> f64 {
    geo_r3_vector_dot(v, v)
}

fn geo_r3_vector_dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

struct geo_s2_Cap {
    center: geo_s2_Point,
    radius: f64,
}

struct geo_s2_Point {
    geo_r3_vector: geo_r3_Vector,
}

struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}
