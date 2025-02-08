
const GEO_S1_RADIAN: f64 = 1.0;

fn angle(c: f64) -> f64 {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if c.is_infinite() {
        return f64::INFINITY;
    }
    return 2.0 * f64::asin(0.5 * f64::sqrt(c));
}

fn is_infinite(c: f64) -> bool {
    f64::is_infinite(c)
}
