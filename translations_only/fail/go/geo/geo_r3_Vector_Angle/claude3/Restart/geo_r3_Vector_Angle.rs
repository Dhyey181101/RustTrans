

use std::f64::consts::PI;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_r3_vector_angle(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoS1Angle {
    let cross = geo_r3_vector_cross(v, ov);
    let dot = geo_r3_vector_dot(v, ov);
    GeoS1Angle(dot.atan2(geo_r3_vector_norm(&cross)) * GEO_S1_RADIAN)
}

fn geo_r3_vector_cross(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.y * ov.z - v.z * ov.y,
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x,
    }
}

fn geo_r3_vector_norm(v: &GeoR3Vector) -> f64 {
    geo_r3_vector_dot(v, v).sqrt()
}

fn geo_r3_vector_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Copy)]
struct GeoS1Angle(f64);

