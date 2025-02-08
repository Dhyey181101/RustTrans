
pub fn contains_interval(i: &Interval, oi: &Interval) -> bool {
    if is_inverted(i) {
        if is_inverted(oi) {
            return oi.lo >= i.lo && oi.hi <= i.hi;
        }
        return (oi.lo >= i.lo || oi.hi <= i.hi) && !is_empty(i);
    }
    if is_inverted(oi) {
        return is_full(i) || is_empty(oi);
    }
    return oi.lo >= i.lo && oi.hi <= i.hi;
}

pub fn is_inverted(i: &Interval) -> bool {
    i.lo > i.hi
}

pub fn is_empty(i: &Interval) -> bool {
    i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI
}

pub fn is_full(i: &Interval) -> bool {
    i.lo == -std::f64::consts::PI && i.hi == std::f64::consts::PI
}

#[derive(Debug)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}
