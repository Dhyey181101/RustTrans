
pub fn contains(i: &geo_s1_interval, p: f64) -> bool {
    let mut p = p;
    if p == -std::f64::consts::PI {
        p = std::f64::consts::PI;
    }
    fast_contains(i, p)
}

pub fn fast_contains(i: &geo_s1_interval, p: f64) -> bool {
    if is_inverted(i) {
        (p >= i.lo || p <= i.hi) && !is_empty(i)
    } else {
        p >= i.lo && p <= i.hi
    }
}

pub fn is_inverted(i: &geo_s1_interval) -> bool {
    i.lo > i.hi
}

pub fn is_empty(i: &geo_s1_interval) -> bool {
    i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI
}

#[derive(Debug)]
pub struct geo_s1_interval {
    pub lo: f64,
    pub hi: f64,
}
