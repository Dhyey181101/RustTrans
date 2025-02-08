
use std::f64::consts::PI;

const GEO_S1_RADIAN: geo_s1_angle = 1.0;
const GEO_S1_DEGREE: geo_s1_angle = (PI / 180.0) * GEO_S1_RADIAN;

fn e7(a: geo_s1_angle) -> i32 {
    geo_s1_round(degrees(a) * 1e7)  
}

fn degrees(a: geo_s1_angle) -> f64 {
    a / GEO_S1_DEGREE as f64
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        ((val - 0.5) as i32)
    } else {
        ((val + 0.5) as i32)
    }
}

struct GeoS1Angle(f64);

type geo_s1_angle = f64;
