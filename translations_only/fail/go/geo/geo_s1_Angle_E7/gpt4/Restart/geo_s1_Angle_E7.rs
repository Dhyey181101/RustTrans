
const GEO_S1_RADIAN: GeoS1Angle = GeoS1Angle(1.0);
const GEO_S1_DEGREE: GeoS1Angle = GeoS1Angle(std::f64::consts::PI / 180.0);

struct GeoS1Angle(f64);

fn e7(angle: &GeoS1Angle) -> i32 {
    geo_s1_round(degrees(angle) * 1e7)
}

fn degrees(angle: &GeoS1Angle) -> f64 {
    angle.0 / GEO_S1_DEGREE.0
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        (val - 0.5) as i32
    } else {
        (val + 0.5) as i32
    }
}

fn main() {}
