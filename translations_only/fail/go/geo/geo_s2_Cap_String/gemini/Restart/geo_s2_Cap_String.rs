
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
    geo_r3_vector: (),
}

#[derive(Debug)]
pub struct GeoR3Vector {}

#[derive(Debug)]
pub struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
    pub fn angle(&self) -> f64 {
        if self.0 < 0.0 {
            return -1.0 * GEO_S1_RADIAN;
        }
        if self.0.is_infinite() {
            return std::f64::INFINITY;
        }
        return 2.0 * f64::asin(0.5 * f64::sqrt(self.0));
    }

    pub fn is_infinity(&self) -> bool {
        return self.0.is_infinite();
    }
}

#[derive(Debug)]
pub struct GeoS1Angle(f64);

impl GeoS1Angle {
    pub fn degrees(&self) -> f64 {
        return self.0 / GEO_S1_DEGREE;
    }
}

pub fn geo_s1_inf_angle() -> GeoS1Angle {
    return GeoS1Angle(std::f64::INFINITY);
}
