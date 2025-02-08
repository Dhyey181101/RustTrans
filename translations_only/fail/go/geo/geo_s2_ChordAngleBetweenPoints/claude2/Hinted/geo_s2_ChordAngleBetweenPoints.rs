

use std::f64::{self};

struct GeoS2Point {
    vec: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct GeoS1ChordAngle(f64);

fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, geo_r3_vector_norm2(&geo_r3_vector_sub(&x.vec, &y.vec))))
}

fn geo_r3_vector_sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn geo_r3_vector_norm2(v: &GeoR3Vector) -> f64 {
    geo_r3_vector_dot(v, v)  
}

fn geo_r3_vector_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

