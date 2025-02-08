
const GEO_S1_RADIAN: f64 = 1.0;

pub fn radius(c: &geo_s2_Cap) -> f64 {
    angle(c.radius)
}

pub fn angle(c: f64) -> f64 {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if c.is_infinite() {
        return f64::INFINITY;
    }
    return 2.0 * f64::asin(0.5 * f64::sqrt(c));
}

pub fn is_infinite(c: f64) -> bool {
    return f64::is_infinite(c);
}

pub fn geo_s1_inf_angle() -> f64 {
    return f64::INFINITY;
}

pub struct geo_s2_Cap {
    pub radius: f64,
}

pub struct geo_s2_Point {
    pub geo_r3_vector: geo_r3_Vector,
}

pub struct geo_r3_Vector {}

pub type geo_s1_ChordAngle = f64;
pub type geo_s1_Angle = f64;
