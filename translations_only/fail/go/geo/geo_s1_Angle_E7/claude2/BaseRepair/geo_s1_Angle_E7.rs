
use std::f64::consts::PI;

struct GeoS1Angle(f64);

fn to_degrees(angle: &GeoS1Angle) -> f64 {
    angle.0 / GEO_S1_DEGREE
}

fn new_geo_s1_angle(radians: f64) -> GeoS1Angle {
    GeoS1Angle(radians)
}

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (PI / 180.0) * GEO_S1_RADIAN;

fn e7(a: &GeoS1Angle) -> i32 {
    geo_s1_round(degrees(a) * 1e7)  
}

fn degrees(a: &GeoS1Angle) -> f64 {
    to_degrees(a)
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        (val - 0.5) as i32
    } else {
        (val + 0.5) as i32
    }
}

