
use std::f64::consts::PI;

const GEO_S1_RADIAN: GeoS1Angle = GeoS1Angle(1.0);

struct GeoS1ChordAngle(f64);

struct GeoS1Angle(f64);

impl GeoS1ChordAngle {
    fn angle(&self) -> GeoS1Angle {
        if self.0 < 0.0 {
            return GeoS1Angle(-1.0 * GEO_S1_RADIAN.0);
        }
        if self.is_infinity() {
            return geo_s1_inf_angle();
        }
        GeoS1Angle(2.0 * (0.5 * self.0).sqrt().asin())
    }

    fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    GeoS1Angle(f64::INFINITY)
}

fn main() {}
