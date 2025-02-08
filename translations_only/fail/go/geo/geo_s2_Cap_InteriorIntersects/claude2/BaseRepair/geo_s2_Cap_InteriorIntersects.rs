

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn interior_intersects(c: Box<GeoS2Cap>, other: Box<GeoS2Cap>) -> bool {
    if c.radius <= 0.0 || is_empty(&other) {
        return false;
    }

    return c.radius + other.radius > chord_angle_between_points(&c.center, &other.center);
}

fn add(c: f64, other: f64) -> f64 {
    if other == 0.0 {
        return c;
    }

    if c + other >= GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }

    let x = (c * (1.0 - 0.25 * other)) as f64;
    let y = (other * (1.0 - 0.25 * c)) as f64;
    return x + y + 2.0 * (x * y).sqrt().min(GEO_S1_MAX_LENGTH2);
}

fn chord_angle_between_points(x: &Box<GeoS2Point>, y: &Box<GeoS2Point>) -> f64 {
    let v = sub(&x.0, &y.0);
    let norm2 = dot(&v, &v);
    return norm2.min(4.0);
}

fn sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z   
}

fn is_empty(c: &GeoS2Cap) -> bool {
    c.radius < 0.0
}

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: f64,
}

struct GeoS2Point(GeoR3Vector);

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

