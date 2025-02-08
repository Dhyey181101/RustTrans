
pub fn interior_contains(i: geo_s1_interval, mut p: f64) -> bool {
    if p == -std::f64::consts::PI {
        p = std::f64::consts::PI;
    }
    if is_inverted(&i) {
        return p > i.lo || p < i.hi;
    }
    (p > i.lo && p < i.hi) || is_full(&i)
}

pub fn is_inverted(i: &geo_s1_interval) -> bool {
    i.lo > i.hi
}

pub fn is_full(i: &geo_s1_interval) -> bool {
    i.lo == -std::f64::consts::PI && i.hi == std::f64::consts::PI
}

#[derive(Debug, Clone, Copy)]
pub struct geo_s1_interval {
    pub lo: f64,
    pub hi: f64,
}
