
pub fn contains_interval(i: &Interval, oi: &Interval) -> bool {
    if oi.lo > oi.hi {
        return true;
    }
    i.lo <= oi.lo && oi.hi <= i.hi
}

pub fn contains(r: &Rect, other: &Rect) -> bool {
    contains_interval(&r.x, &other.x) && contains_interval(&r.y, &other.y)
}

#[derive(Debug)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

#[derive(Debug)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}
