
use std::f64::{self};

#[derive(PartialEq, PartialOrd)]
struct GeoS1ChordAngle(f64);

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    vector: GeoR3Vector,  
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn contains_point(c: &GeoS2Cap, p: &GeoS2Point) -> bool {
    chord_angle_between_points(&c.center, p) <= c.radius
}

fn chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, vector_norm2(&sub_vectors(&x.vector, &y.vector))))
}

fn sub_vectors(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}  

fn vector_norm2(v: &GeoR3Vector) -> f64 {
    vector_dot(v, v)
}

fn vector_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z  
}

