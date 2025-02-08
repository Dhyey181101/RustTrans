
use std::f64;
use std::ops::Neg;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s1_angle(c: f64) -> f64 {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if c.is_infinite() {
        return f64::INFINITY;
    }
    return 2.0 * f64::asin(0.5 * f64::sqrt(0.5 * c * c));
}

struct GeoS2Cap {
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector;

struct GeoS1ChordAngle(f64);

struct GeoS1Angle(f64);

impl Neg for GeoS1ChordAngle {
    type Output = Self;
    fn neg(self) -> Self::Output {
        GeoS1ChordAngle(-self.0)
    }
}

impl GeoS1Angle {
    fn angle(&self) -> GeoS1Angle {
        GeoS1Angle(geo_s1_angle(self.0))
    }
}

impl GeoS1ChordAngle {
    fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }

    fn inf_angle() -> GeoS1Angle {
        GeoS1Angle(f64::INFINITY)
    }
}
