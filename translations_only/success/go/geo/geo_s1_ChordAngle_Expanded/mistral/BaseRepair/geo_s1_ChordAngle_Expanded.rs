
use std::isize;
use std::f64;

const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn geo_s1_expanded(c: geo_s1_ChordAngle, e: f64) -> geo_s1_ChordAngle {
    if c.is_special() {
        return c;
    }
    geo_s1_ChordAngle(f64::max(0.0, f64::min(GEO_S1_MAX_LENGTH2, c.0 + e)))
}

struct geo_s1_ChordAngle(f64);

impl geo_s1_ChordAngle {
    fn is_special(&self) -> bool {
        self.0 < 0.0 || self.is_infinity()
    }

    fn is_infinity(&self) -> bool {
        self.0.is_infinite() && self.0.is_sign_positive()
    }
}
