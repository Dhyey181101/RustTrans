

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = PI;

fn geo_s2_cap_interior_contains_point(cap: &Box<GeoS2Cap>, p: &GeoS2Point) -> bool {
    return geo_s2_cap_is_full(cap) || geo_s2_chord_angle_between_points(&cap.center, p) < cap.radius;
}

fn geo_s2_cap_is_full(cap: &Box<GeoS2Cap>) -> bool {
    return cap.radius == GEO_S1_STRAIGHT_CHORD_ANGLE;
}

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> f64 {
    let diff = geo_r3_vector_sub(&x.geo_r3_vector, &y.geo_r3_vector);
    return f64::min(PI * 2.0, geo_r3_vector_norm2(&diff));
}

fn geo_r3_vector_sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> Box<GeoR3Vector> {
    let result = Box::new(GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    });
    return result;
}

fn geo_r3_vector_norm2(v: &GeoR3Vector) -> f64 {
    return geo_r3_vector_dot(v, v);
}

fn geo_r3_vector_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    return (v.x * ov.x) + (v.y * ov.y) + (v.z * ov.z);
}

struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

