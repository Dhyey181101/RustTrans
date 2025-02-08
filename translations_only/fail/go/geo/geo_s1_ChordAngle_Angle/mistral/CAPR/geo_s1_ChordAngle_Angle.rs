
use std::f64;
use std::isize;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s1_angle(c: f64) -> f64 {
    if c < 0.0 {
        -1.0 * GEO_S1_RADIAN
    } else if c.is_infinite() {
        f64::INFINITY
    } else {
        2.0 * f64::asin(0.5 * f64::sqrt(c))
    }
}

fn geo_s1_is_infinity(c: f64) -> bool {
    c.is_infinite()
}

#[derive(Copy, Clone, Debug)]
struct GeoS1ChordAngle(f64);

#[derive(Copy, Clone, Debug)]
struct GeoS1Angle(f64);

impl GeoS1ChordAngle {
    fn angle(&self) -> GeoS1Angle {
        GeoS1Angle(geo_s1_angle(self.0))
    }
}

impl GeoS1Angle {
    fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}
