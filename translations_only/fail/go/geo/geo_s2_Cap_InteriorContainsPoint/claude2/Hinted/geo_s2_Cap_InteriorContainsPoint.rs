
use std::f64::consts::PI;

#[derive(PartialEq, PartialOrd)]
struct GeoS1ChordAngle(f64);

const GEO_S1_STRAIGHT_CHORD_ANGLE: GeoS1ChordAngle = GeoS1ChordAngle(4.0);

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    v: Box<GeoR3Vector>,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn geo_s2_cap_interior_contains_point(cap: &GeoS2Cap, p: &GeoS2Point) -> bool {
    geo_s2_cap_is_full(cap) || geo_s2_chord_angle_between_points(&cap.center, p) < cap.radius
}

fn geo_s2_cap_is_full(cap: &GeoS2Cap) -> bool {
    cap.radius == GEO_S1_STRAIGHT_CHORD_ANGLE
}

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(geo_r3_vector_norm2(&sub(&*x.v, &*y.v)).min(4.0))
}

fn sub(x: &GeoR3Vector, y: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: x.x - y.x,
        y: x.y - y.y,
        z: x.z - y.z,
    }
}

fn geo_r3_vector_norm2(v: &GeoR3Vector) -> f64 {
    geo_r3_vector_dot(v, v)  
}

fn geo_r3_vector_dot(x: &GeoR3Vector, y: &GeoR3Vector) -> f64 {
    x.x * y.x + x.y * y.y + x.z * y.z
}

