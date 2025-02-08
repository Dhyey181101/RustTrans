
pub fn intersects(i: geo_s1_Interval, oi: geo_s1_Interval) -> bool {
    if i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI || oi.lo == std::f64::consts::PI && oi.hi == -std::f64::consts::PI {
        return false;
    }
    if i.lo > i.hi {
        return oi.lo <= i.hi || oi.hi >= i.lo;
    }
    if oi.lo > oi.hi {
        return oi.lo <= i.hi || oi.hi >= i.lo;
    }
    oi.lo <= i.hi && oi.hi >= i.lo
}

pub fn is_empty(i: &geo_s1_Interval) -> bool {
    i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI
}

#[derive(Debug, Clone, Copy)]
pub struct geo_s1_Interval {
    pub lo: f64,
    pub hi: f64,
}
