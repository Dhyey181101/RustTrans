
use std::f64::consts::PI;

fn geo_s2_cap_centroid(c: Box<geo_s2_Cap>) -> geo_s2_Point {
    if geo_s2_cap_is_empty(&c) {
        return geo_s2_Point { geo_r3_vector: geo_r3_Vector { x: 0.0, y: 0.0, z: 0.0 } };
    }
    let r = 1.0 - 0.5 * geo_s2_cap_height(&c);
    let area = geo_s2_cap_area(&c);
    geo_s2_Point { geo_r3_vector: geo_r3_vector_mul(&c.center.geo_r3_vector, r * area) }
}

fn geo_s2_cap_is_empty(c: &geo_s2_Cap) -> bool {
    c.radius < 0.0
}

fn geo_s2_cap_height(c: &geo_s2_Cap) -> f64 {
    c.radius
}

fn geo_s2_cap_area(c: &geo_s2_Cap) -> f64 {
    2.0 * PI * f64::max(0.0, geo_s2_cap_height(c))
}

fn geo_r3_vector_mul(v: &geo_r3_Vector, m: f64) -> geo_r3_Vector {
    geo_r3_Vector { x: m * v.x, y: m * v.y, z: m * v.z }
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

