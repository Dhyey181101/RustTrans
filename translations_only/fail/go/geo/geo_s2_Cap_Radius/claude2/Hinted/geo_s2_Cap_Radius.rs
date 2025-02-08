
use std::f64::INFINITY;

const GEO_S1_RADIAN: f64 = 1.0;

fn radius(c: GeoS2Cap) -> f64 {
    angle(c.radius)
}

fn angle(c: f64) -> f64 {
    if c < 0.0 {
        -1.0 * GEO_S1_RADIAN
    } else if c.is_infinite() {
        f64::INFINITY
    } else {
        2.0 * f64::asin(0.5 * c.sqrt())
    }
}

struct GeoS2Cap {
    radius: f64
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector   
}

struct GeoR3Vector;

