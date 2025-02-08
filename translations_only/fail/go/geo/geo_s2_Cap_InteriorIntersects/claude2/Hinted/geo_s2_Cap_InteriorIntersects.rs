

use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn interior_intersects(c: &GeoS2Cap, other: &GeoS2Cap) -> bool {
    if c.radius <= 0.0 || is_empty(&other) {
        return false;
    }

    let chord_angle = geo_s2_chord_angle_between_points(&c.center, &other.center);
    c.radius + other.radius > chord_angle
}

fn add(c: f64, other: f64) -> f64 {
    if other == 0.0 {
        return c;
    }

    if c + other >= GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }

    let x = c * (1.0 - 0.25 * other);
    let y = other * (1.0 - 0.25 * c);

    f64::min(GEO_S1_MAX_LENGTH2, x + y + 2.0 * (x * y).sqrt())
}

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> f64 {
    f64::min(4.0, geo_r3_vector_norm2(&sub_geo_r3_vectors(&x.vector, &y.vector)))  
}

fn sub_geo_r3_vectors(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
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

fn is_empty(c: &GeoS2Cap) -> bool {
    c.radius < 0.0 
}

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: f64,
}

struct GeoS2Point {
    vector: GeoR3Vector,  
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,   
}

