
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
        write!(f, "[Center={:?}, Radius={:.2}]", self.center.geo_r3_vector, self.radius.0.to_radians())
    }
}

impl GeoS2Cap {
    fn radius(&self) -> GeoS1Angle {
        GeoS1Angle(self.radius.0.to_radians())
    }
}

impl GeoS1ChordAngle {
    fn to_radians(&self) -> f64 {
        if self.0 < 0.0 {
            -1.0 * GEO_S1_RADIAN
        } else if self.0.is_infinite() {
            f64::INFINITY
        } else {
            2.0 * (f64::consts::FRAC_PI_2).sin() * (0.5 * (self.0).sqrt())
        }
    }

    fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}

impl GeoS1Angle {
    fn to_degrees(&self) -> f64 {
        self.0 / GEO_S1_DEGREE
    }
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    GeoS1Angle(f64::INFINITY)
}
