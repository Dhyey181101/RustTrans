
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

fn interior_contains_point(c: Box<GeoS2Cap>, p: GeoS2Point) -> bool {
    is_full(&c) || geo_s2_chord_angle_between_points(c.center, p) < c.radius
}

fn is_full(c: &GeoS2Cap) -> bool {
    c.radius == GEO_S1_STRAIGHT_CHORD_ANGLE  
}

fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> f64 {
    geo_s1_chord_angle_min(4.0, geo_r3_vector_norm2(&geo_r3_vector_sub(&x.v, &y.v)))
}

fn geo_r3_vector_sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z
    }
}

fn geo_r3_vector_norm2(v: &GeoR3Vector) -> f64 {
    geo_r3_vector_dot(v, v)  
}

fn geo_r3_vector_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

fn geo_s1_chord_angle_min(x: f64, y: f64) -> f64 {
    x.min(y)
}

struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64
}

struct GeoS2Point {
    v: GeoR3Vector
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64  
}

