
pub fn interior_intersects(i: &Interval, oi: &Interval) -> bool {
    if is_empty(i) || is_empty(oi) || i.lo == i.hi {
        return false;
    }
    if is_inverted(i) {
        return oi.lo < i.hi || oi.hi > i.lo || is_inverted(oi);
    }
    if is_inverted(oi) {
        return oi.lo < i.hi || oi.hi > i.lo;
    }
    (oi.lo < i.hi && oi.hi > i.lo) || is_full(i)
}

pub fn is_empty(i: &Interval) -> bool {
    i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI
}

pub fn is_inverted(i: &Interval) -> bool {
    i.lo > i.hi
}

pub fn is_full(i: &Interval) -> bool {
    i.lo == -std::f64::consts::PI && i.hi == std::f64::consts::PI
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}
