
use std::f64;

fn geo_s2_cap_contains_point(c: &geo_s2_cap, p: &geo_s2_point) -> bool {
    geo_s2_chord_angle_between_points(&c.center, p) <= c.radius
}

fn geo_s2_chord_angle_between_points(x: &geo_s2_point, y: &geo_s2_point) -> f64 {
    geo_s1_chord_angle_fn(f64::min(4.0, geo_r3_vector_distance(&x.geo_r3_vector, &y.geo_r3_vector)))
}

fn geo_r3_vector_distance(v: &geo_r3_vector, ov: &geo_r3_vector) -> f64 {
    let dx = v.x - ov.x;
    let dy = v.y - ov.y;
    let dz = v.z - ov.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn geo_r3_vector_sub(v: &geo_r3_vector, ov: &geo_r3_vector) -> geo_r3_vector {
    geo_r3_vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn geo_r3_vector_dot(v: &geo_r3_vector, ov: &geo_r3_vector) -> f64 {
    (v.x * ov.x) + (v.y * ov.y) + (v.z * ov.z)
}

fn geo_s1_chord_angle_fn(x: f64) -> f64 {
    if x < 0.5 {
        x
    } else if x < 1.0 {
        (std::f64::consts::PI / 4.0) * (x + 1.0).powi(2)
    } else {
        0.5 * std::f64::consts::PI - 0.5 * (4.0 - x).sqrt()
    }
}

#[derive(Clone, Copy)]
struct geo_s2_cap {
    center: geo_s2_point,
    radius: f64,
}

#[derive(Clone, Copy)]
struct geo_s2_point {
    geo_r3_vector: geo_r3_vector,
}

#[derive(Clone, Copy)]
struct geo_r3_vector {
    x: f64,
    y: f64,
    z: f64,
}
