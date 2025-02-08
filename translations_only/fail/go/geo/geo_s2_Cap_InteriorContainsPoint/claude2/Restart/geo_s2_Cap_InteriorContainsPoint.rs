
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

fn interior_contains_point(c: Box<GeoS2Cap>, p: GeoS2Point) -> bool {
    is_full(&c) || geo_s2_chord_angle_between_points(c.center, p) < c.radius
}

fn is_full(c: &GeoS2Cap) -> bool {
    c.radius == GEO_S1_STRAIGHT_CHORD_ANGLE  
}

fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> f64 {
    f64::min(GEO_S1_STRAIGHT_CHORD_ANGLE, vector_norm2(&vector_sub(&x.v, &y.v)))
}

fn vector_sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z
    }
}

fn vector_norm2(v: &GeoR3Vector) -> f64 {
    vector_dot(v, v)  
}

fn vector_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
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

