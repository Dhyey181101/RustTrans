

use std::f64::consts::PI;

const GEO_S1_RADIAN: geo_s1_angle = geo_s1_angle(1.0);
const GEO_S1_DEGREE: geo_s1_angle = geo_s1_angle((PI / 180.0) * GEO_S1_RADIAN.0);

fn e6(a: geo_s1_angle) -> i32 {
    geo_s1_round(geo_s1_to_degrees(a) * 1e6) 
}

fn geo_s1_to_degrees(angle: geo_s1_angle) -> f64 {
    angle.0 / GEO_S1_DEGREE.0  
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        return (val - 0.5) as i32;
    }
    (val + 0.5) as i32
}

struct geo_s1_angle(f64);

