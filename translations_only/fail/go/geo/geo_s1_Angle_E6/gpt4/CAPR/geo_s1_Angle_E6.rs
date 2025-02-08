
const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = std::f64::consts::PI / 180.0 * GEO_S1_RADIAN;

struct GeoS1Angle(f64);

impl GeoS1Angle {
    fn e6(&self) -> i32 {
        geo_s1_round(self.degrees() * 1e6)
    }

    fn degrees(&self) -> f64 {
        self.0 / GEO_S1_DEGREE
    }
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        (val - 0.5) as i32
    } else {
        (val + 0.5) as i32
    }
}
