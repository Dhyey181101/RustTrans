
use std::f64::consts::PI;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

fn geo_s2_cap_centroid(c: Box<GeoS2Cap>) -> GeoS2Point {
    if geo_s2_cap_is_empty(&c) {
        return GeoS2Point { geo_r3_vector: GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 } };
    }
    let r = 1.0 - 0.5 * geo_s2_cap_height(&c);
    let area = geo_s2_cap_area(&c);
    let mut center = c.center.geo_r3_vector;
    center = geo_r3_vector_mul(center, r * area);
    GeoS2Point { geo_r3_vector: center }
}

fn geo_s2_cap_is_empty(c: &GeoS2Cap) -> bool {
    c.radius < 0.0
}

fn geo_s2_cap_height(c: &GeoS2Cap) -> f64 {
    0.5 * c.radius
}

fn geo_s2_cap_area(c: &GeoS2Cap) -> f64 {
    2.0 * PI * f64::max(0.0, geo_s2_cap_height(c))
}

fn geo_r3_vector_mul(v: GeoR3Vector, m: f64) -> GeoR3Vector {
    GeoR3Vector { x: m * v.x, y: m * v.y, z: m * v.z }
}
