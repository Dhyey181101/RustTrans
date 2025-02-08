
pub fn intersects(i: geo_s1_Interval, oi: geo_s1_Interval) -> bool {
    if is_empty(&i) || is_empty(&oi) {
        return false;
    }
    if is_inverted(&i) {
        return is_inverted(&oi) || oi.lo <= i.hi || oi.hi >= i.lo;
    }
    if is_inverted(&oi) {
        return oi.lo <= i.hi || oi.hi >= i.lo;
    }
    oi.lo <= i.hi && oi.hi >= i.lo
}

pub fn is_empty(i: &geo_s1_Interval) -> bool {
    i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI
}

pub fn is_inverted(i: &geo_s1_Interval) -> bool {
    i.lo > i.hi
}

#[derive(Debug, Clone, Copy)]
pub struct geo_s1_Interval {
    pub lo: f64,
    pub hi: f64,
}
