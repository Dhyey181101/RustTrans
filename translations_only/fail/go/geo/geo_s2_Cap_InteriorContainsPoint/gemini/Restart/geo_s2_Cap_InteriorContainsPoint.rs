
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

pub fn geo_s2_cap_interior_contains_point(cap: &GeoS2Cap, p: &GeoS2Point) -> bool {
    cap.radius == GEO_S1_STRAIGHT_CHORD_ANGLE || geo_s2_chord_angle_between_points(&cap.center, p) < cap.radius
}

pub fn geo_s2_cap_is_full(cap: &GeoS2Cap) -> bool {
    cap.radius == GEO_S1_STRAIGHT_CHORD_ANGLE
}

pub fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> f64 {
    f64::min(4.0, geo_r3_vector_norm2(&geo_r3_vector_sub(&x.geo_r3_vector, &y.geo_r3_vector)))
}

pub fn geo_r3_vector_sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

pub fn geo_r3_vector_norm2(v: &GeoR3Vector) -> f64 {
    geo_r3_vector_dot(v, v)
}

pub fn geo_r3_vector_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Debug)]
pub struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64,
}

#[derive(Debug)]
pub struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

#[derive(Debug)]
pub struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

pub type GeoS1ChordAngle = f64;
