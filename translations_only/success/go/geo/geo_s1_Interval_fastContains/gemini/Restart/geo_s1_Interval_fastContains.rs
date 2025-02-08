
pub fn fast_contains(interval: &geo_s1_Interval, p: f64) -> bool {
    if is_inverted(interval) {
        return (p >= interval.lo || p <= interval.hi) && !is_empty(interval);
    }
    p >= interval.lo && p <= interval.hi
}

pub fn is_inverted(interval: &geo_s1_Interval) -> bool {
    interval.lo > interval.hi
}

pub fn is_empty(interval: &geo_s1_Interval) -> bool {
    interval.lo == std::f64::consts::PI && interval.hi == -std::f64::consts::PI
}

#[derive(Debug, Clone, Copy)]
pub struct geo_s1_Interval {
    pub lo: f64,
    pub hi: f64,
}
