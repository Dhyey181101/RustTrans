
const GEO_S1_RADIAN: f64 = 1.0;

pub fn angle(c: f64) -> f64 {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if c.is_infinite() {
        return f64::INFINITY;
    }
    return 2.0 * f64::asin(0.5 * f64::sqrt(c));
}

pub fn is_infinity(c: f64) -> bool {
    return c.is_infinite();
}

pub fn inf_angle() -> f64 {
    return f64::INFINITY;
}
