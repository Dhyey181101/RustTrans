
use std::f64::consts::PI;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct GeoS1ChordAngle(f64);

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    vector: GeoR3Vector,
}

#[derive(Copy, Clone)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn geo_s2_cap_contains_point(c: &GeoS2Cap, p: GeoS2Point) -> bool {
    geo_s1_chord_angle_between_points(&*c.center, p) <= c.radius
}

fn geo_s1_chord_angle_between_points(x: &GeoS2Point, y: GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, geo_r3_vector_distance(x.vector, y.vector)))  
}

fn geo_r3_vector_distance(v: GeoR3Vector, ov: GeoR3Vector) -> f64 {
    geo_r3_vector_norm(&geo_r3_vector_sub(v, ov))
}

fn geo_r3_vector_sub(v: GeoR3Vector, ov: GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn geo_r3_vector_norm(v: &GeoR3Vector) -> f64 {
    (geo_r3_vector_dot(v, v)).sqrt()
}

fn geo_r3_vector_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

