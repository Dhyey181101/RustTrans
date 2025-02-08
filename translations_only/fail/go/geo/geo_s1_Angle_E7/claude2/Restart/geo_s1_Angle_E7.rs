
use std::f64::consts::PI;

struct GeoS1Angle(f64);

const GEO_S1_RADIAN: GeoS1Angle = GeoS1Angle(1.0);
const GEO_S1_DEGREE: GeoS1Angle = GeoS1Angle(PI / 180.0);

fn to_degrees(angle: &GeoS1Angle) -> f64 {
    angle.0 * 180.0 / PI
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        ((val - 0.5) as i32)
    } else {
        ((val + 0.5) as i32)
    }
}

fn e7(a: &GeoS1Angle) -> i32 {
    geo_s1_round(to_degrees(a) * 1e7)  
}

