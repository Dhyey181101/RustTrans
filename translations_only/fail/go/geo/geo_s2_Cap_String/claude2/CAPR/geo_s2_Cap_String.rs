
use std::f64::{self, INFINITY};
use std::fmt;
use std::ops::Mul;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (std::f64::consts::PI / 180.0) * GEO_S1_RADIAN;

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: geo_s1_ChordAngle,
}

impl fmt::Display for GeoS2Cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let center_str = format!("{:?}", *self.center);
        write!(f, "[Center={}, Radius={}]", center_str, degrees(radius(self)))
    }
}

#[derive(Debug)]
struct GeoS2Point {
    vector: GeoR3Vector,
}

#[derive(Debug)]
struct GeoR3Vector;

fn radius(cap: &GeoS2Cap) -> geo_s1_Angle {
    angle(&cap.radius)
}

fn angle(ca: &geo_s1_ChordAngle) -> geo_s1_Angle {
    if *ca < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if is_infinity(ca) {
        return geo_s1_inf_angle();
    }
    2.0 * ca.sin().sqrt() / 2.0
}

fn is_infinity(ca: &geo_s1_ChordAngle) -> bool {
    ca.is_infinite()
}

fn geo_s1_inf_angle() -> geo_s1_Angle {
    INFINITY
}

fn degrees(a: geo_s1_Angle) -> f64 {
    a / GEO_S1_DEGREE
}

type geo_s1_ChordAngle = f64;
type geo_s1_Angle = f64;

