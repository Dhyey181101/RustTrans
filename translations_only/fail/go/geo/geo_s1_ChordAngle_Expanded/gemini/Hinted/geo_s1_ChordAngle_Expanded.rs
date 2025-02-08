
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn expanded(c: f64, e: f64) -> f64 {
    if is_special(c) {
        return c;
    }
    f64::max(0.0, f64::min(GEO_S1_MAX_LENGTH2, c + e))
}

fn is_special(c: f64) -> bool {
    c < 0.0 || c.is_infinite()
}
