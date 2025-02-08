
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn interior_intersects(c: Box<GeoS2Cap>, other: Box<GeoS2Cap>) -> bool {
    if get_radius(&c) <= 0.0 || is_empty(&other) {
        return false;
    }

    return get_radius(&c) + get_radius(&other) > chord_angle_between_points(get_center(&c), get_center(&other));
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

fn chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> f64 {
    let v = sub(&get_vector(&x), &get_vector(&y));
    let n = norm2(&v);
    return n.min(4.0);
}

fn sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn norm2(v: &GeoR3Vector) -> f64 {
    return dot(v, v);
}

fn dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z  
}

fn is_empty(c: &GeoS2Cap) -> bool {
    get_radius(c) < 0.0
}

fn get_radius(c: &GeoS2Cap) -> f64 {
    c.radius
}

fn get_center(c: &GeoS2Cap) -> &GeoS2Point {
    &c.center
}

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: f64,
}

struct GeoS2Point(GeoR3Vector);

fn get_vector(p: &GeoS2Point) -> &GeoR3Vector {
    &p.0
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64  
}

