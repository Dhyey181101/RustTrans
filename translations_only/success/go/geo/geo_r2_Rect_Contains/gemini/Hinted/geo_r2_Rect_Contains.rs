
use std::cmp::{max, min};

fn contains_interval(i: &Interval, oi: &Interval) -> bool {
    if oi.is_empty() {
        return true;
    }
    i.lo <= oi.lo && oi.hi <= i.hi
}

fn contains(r: &Rect, other: &Rect) -> bool {
    contains_interval(&r.x, &other.x) && contains_interval(&r.y, &other.y)
}

struct Interval {
    lo: f64,
    hi: f64,
}

impl Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

struct Rect {
    x: Interval,
    y: Interval,
}
