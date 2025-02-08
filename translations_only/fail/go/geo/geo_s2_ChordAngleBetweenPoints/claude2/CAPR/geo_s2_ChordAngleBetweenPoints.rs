
use std::f64::MIN;

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
    GeoS1ChordAngle(geo_min(4.0, geo_norm2(&x.vec, &y.vec)))  
}

fn geo_sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn geo_norm2(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    geo_dot(v, ov)
}

fn geo_dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z  
}

fn geo_min(a: f64, b: f64) -> f64 {
    if a < b {a} else {b}
}

