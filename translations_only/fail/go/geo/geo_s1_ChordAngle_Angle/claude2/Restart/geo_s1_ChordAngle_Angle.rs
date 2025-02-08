
use std::f64::INFINITY;

struct GeoS1Angle(f64);

impl GeoS1Angle {
    fn new(angle: f64) -> GeoS1Angle {
        GeoS1Angle(angle)
    }
}

fn geo_s1_radian() -> GeoS1Angle {
    GeoS1Angle::new(1.0)  
}

struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {

    fn angle(&self) -> GeoS1Angle {
        if self.0 < 0.0 {
            return GeoS1Angle::new(-1.0 * geo_s1_radian().0);
        }
        if self.is_infinity() {
            return geo_s1_inf_angle();
        }
        GeoS1Angle::new(2.0 * (0.5 * self.0).asin())
    }

    fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    GeoS1Angle::new(INFINITY)
}

