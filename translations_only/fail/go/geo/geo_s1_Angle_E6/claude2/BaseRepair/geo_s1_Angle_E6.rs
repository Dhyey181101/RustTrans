
use std::f64::consts::PI;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (PI / 180.0) * GEO_S1_RADIAN;

fn e6(a: f64) -> i32 {
    geo_s1_round(to_degrees(a) * 1e6)  
}

fn to_degrees(angle: f64) -> f64 {
    angle / GEO_S1_DEGREE
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        (val - 0.5) as i32
    } else {
        (val + 0.5) as i32
    }
}

struct GeoS1Angle(f64);

