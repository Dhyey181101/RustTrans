

use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

#[derive(Clone, Copy)]
struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

#[derive(Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

fn geo_s2_cap_add_point(mut c: GeoS2Cap, p: GeoS2Point) -> GeoS2Cap {
    if geo_s2_cap_is_empty(c) {
        c.center = p;
        c.radius = 0.0;
        return c;
    }

    let new_rad = geo_s2_chord_angle_between_points(c.center, p);
    if new_rad > c.radius {
        c.radius = new_rad;
    }
    c
}

fn geo_s2_cap_is_empty(c: GeoS2Cap) -> bool {
    c.radius < 0.0
}

fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> GeoS1ChordAngle {
    let diff = geo_r3_vector_sub(x.geo_r3_vector, y.geo_r3_vector);
    let norm2 = geo_r3_vector_dot(diff, diff);
    geo_s1_chord_angle(norm2.min(4.0))
}

fn geo_r3_vector_sub(v: GeoR3Vector, ov: GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn geo_r3_vector_dot(v: GeoR3Vector, ov: GeoR3Vector) -> f64 {
    (v.x * ov.x) + (v.y * ov.y) + (v.z * ov.z)
}

fn geo_s1_chord_angle(x: f64) -> GeoS1ChordAngle {
    x * PI / 2.0
}

