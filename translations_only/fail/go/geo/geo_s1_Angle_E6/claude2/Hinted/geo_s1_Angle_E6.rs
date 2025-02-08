
use std::f64::consts::PI;

fn new_geo_s1_angle(v: f64) -> Box<GeoS1Angle> {
    Box::new(GeoS1Angle {
        0: v,
    })
}

struct GeoS1Angle(f64);

static GEO_S1_RADIAN: GeoS1Angle = GeoS1Angle(1.0);
static GEO_S1_DEGREE: GeoS1Angle = GeoS1Angle((PI / 180.0) * GEO_S1_RADIAN.0);

fn e6(a: &GeoS1Angle) -> i32 {
    geo_s1_round(degrees(a.0) * 1e6)  
}

fn degrees(a: f64) -> f64 {
    a / GEO_S1_DEGREE.0
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        return (val - 0.5) as i32;
    }
    (val + 0.5) as i32
}

