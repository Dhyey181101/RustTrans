
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64,
}

struct GeoS2Point {
    vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64, 
    z: f64,
}

fn is_valid(cap: &GeoS2Cap) -> bool {
    is_unit(&cap.center.vector) && cap.radius <= GEO_S1_STRAIGHT_CHORD_ANGLE
}

fn is_unit(v: &GeoR3Vector) -> bool {
    const EPSILON: f64 = 5e-14;
    (norm2(v) - 1.0).abs() <= EPSILON
}

fn norm2(v: &GeoR3Vector) -> f64 {
    dot(v, v)  
}

fn dot(v1: &GeoR3Vector, v2: &GeoR3Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}
