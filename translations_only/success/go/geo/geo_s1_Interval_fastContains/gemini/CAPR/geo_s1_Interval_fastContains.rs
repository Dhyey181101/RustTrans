
pub fn fast_contains(i: &geo_s1_Interval, p: f64) -> bool {
    if is_inverted(i) {
        return (p >= i.lo || p <= i.hi) && !is_empty(i);
    }
    return p >= i.lo && p <= i.hi;
}

pub fn is_inverted(i: &geo_s1_Interval) -> bool {
    i.lo > i.hi
}

pub fn is_empty(i: &geo_s1_Interval) -> bool {
    i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI
}

pub struct geo_s1_Interval {
    pub lo: f64,
    pub hi: f64,
}
