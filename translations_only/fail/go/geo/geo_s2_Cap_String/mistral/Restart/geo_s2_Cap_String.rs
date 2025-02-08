
use std::f64;
use std::fmt;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (std::f64::consts::PI / 180.0) * GEO_S1_RADIAN;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: (),
}

struct GeoR3Vector {
}

struct GeoS1ChordAngle(f64);

struct GeoS1Angle(f64);

impl fmt::Display for GeoS2Cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Center={:?}, Radius={:?}]", self.center.geo_r3_vector, self.radius.0)
    }
}

impl GeoS2Cap {
    fn radius(&self) -> GeoS1Angle {
        GeoS1Angle(self.radius.0.to_radians())
    }
}

impl GeoS1ChordAngle {
    fn angle(&self) -> GeoS1Angle {
        if self.0 < 0.0 {
            return GeoS1Angle(-1.0 * GEO_S1_RADIAN);
        }
        if self.is_infinity() {
            return GeoS1Angle(f64::INFINITY);
        }
        GeoS1Angle(2.0 * (f64::asin(0.5 * (self.0).sqrt())))
    }

    fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}

impl GeoS1Angle {
    fn degrees(&self) -> f64 {
        self.0 / GEO_S1_DEGREE
    }
}

impl GeoS1ChordAngle {
    fn to_radians(&self) -> f64 {
        self.0 * (std::f64::consts::PI / 180.0)
    }
}

impl GeoS1Angle {
    fn to_radians(&self) -> f64 {
        self.0 * (std::f64::consts::PI / 180.0)
    }
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    GeoS1Angle(f64::INFINITY)
}
