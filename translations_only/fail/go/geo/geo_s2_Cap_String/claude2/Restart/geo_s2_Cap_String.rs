
use std::f64::{self, INFINITY};
use std::fmt;

#[derive(Debug)]
struct GeoS2Point {
    vector: Box<GeoR3Vector>,
}

#[derive(Debug)]
struct GeoR3Vector {

}

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (std::f64::consts::PI / 180.0) * GEO_S1_RADIAN;

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: f64,
}

impl fmt::Display for GeoS2Cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Center={:?}, Radius={}]", *self.center, degrees(radius(self)))
    }
}

fn radius(cap: &GeoS2Cap) -> f64 {
    angle(&cap.radius)
}

fn angle(ca: &f64) -> f64 {
    if *ca < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if is_infinity(ca) {
        return geo_s1_inf_angle();
    }
    2.0 * (0.5 * ca.sin()).asin()
}

fn is_infinity(ca: &f64) -> bool {
    ca.is_infinite()
}

fn geo_s1_inf_angle() -> f64 {
    f64::INFINITY
}

fn degrees(a: f64) -> f64 {
    a / GEO_S1_DEGREE
}

