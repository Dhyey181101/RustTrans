
pub fn interior_contains_interval(i: &geo_s1_interval, oi: &geo_s1_interval) -> bool {
    if is_inverted(i) {
        if is_inverted(oi) {
            return (oi.lo > i.lo && oi.hi < i.hi) || is_empty(oi)
        }
        return oi.lo > i.lo || oi.hi < i.hi
    }
    if is_inverted(oi) {
        return is_full(i) || is_empty(oi)
    }
    return (oi.lo > i.lo && oi.hi < i.hi) || is_full(i)
}

pub fn is_inverted(i: &geo_s1_interval) -> bool {
    i.lo > i.hi
}

pub fn is_empty(i: &geo_s1_interval) -> bool {
    i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI
}

pub fn is_full(i: &geo_s1_interval) -> bool {
    i.lo == -std::f64::consts::PI && i.hi == std::f64::consts::PI
}

#[derive(Debug, Clone, Copy)]
pub struct geo_s1_interval {
    pub lo: f64,
    pub hi: f64,
}
