
const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (std::f64::consts::PI / 180.0) * GEO_S1_RADIAN;

#[derive(Debug)]
pub struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    pub fn radius(&self) -> f64 {
        self.radius.angle()
    }
}

#[derive(Debug)]
pub struct GeoS2Point {
    vector: GeoR3Vector,
}

#[derive(Debug)]
pub struct GeoR3Vector {}

#[derive(Debug)]
pub struct GeoS1ChordAngle(pub f64);

impl GeoS1ChordAngle {
    pub fn angle(&self) -> f64 {
        if self.0 < 0.0 {
            return -1.0 * GEO_S1_RADIAN;
        }
        if self.0.is_infinite() {
            return f64::INFINITY;
        }
        return 2.0 * f64::asin(0.5 * f64::sqrt(self.0));
    }

    pub fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}

#[derive(Debug)]
pub struct GeoS1Angle(pub f64);

impl GeoS1Angle {
    pub fn degrees(&self) -> f64 {
        self.0 / GEO_S1_DEGREE
    }
}

pub fn geo_s1_inf_angle() -> f64 {
    f64::INFINITY
}
