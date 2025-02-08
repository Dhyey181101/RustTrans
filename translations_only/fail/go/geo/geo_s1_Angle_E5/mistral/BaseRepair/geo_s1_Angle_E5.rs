
use std::f64;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (f64::consts::PI / 180.0) * GEO_S1_RADIAN;

fn e5(a: f64) -> i32 {
    geo_s1_round(a * 1e5)
}

fn degrees(a: f64) -> f64 {
    a / GEO_S1_DEGREE
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        val as i32 - 1
    } else {
        val as i32 + 1
    }
}

type GeoS1Angle = f64;
