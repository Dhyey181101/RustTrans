
pub fn intersects(i: geo_s1_interval, oi: geo_s1_interval) -> bool {
    if i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI || oi.lo == std::f64::consts::PI && oi.hi == -std::f64::consts::PI {
        return false;
    }
    if i.lo > i.hi {
        return oi.lo > oi.hi || oi.lo <= i.hi || oi.hi >= i.lo;
    }
    if oi.lo > oi.hi {
        return oi.lo <= i.hi || oi.hi >= i.lo;
    }
    oi.lo <= i.hi && oi.hi >= i.lo
}

pub fn is_empty(i: geo_s1_interval) -> bool {
    i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI
}

pub fn is_inverted(i: geo_s1_interval) -> bool {
    i.lo > i.hi
}

#[derive(Debug, Clone, Copy)]
pub struct geo_s1_interval {
    pub lo: f64,
    pub hi: f64,
}
