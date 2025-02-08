
use std::f64::consts::PI;

pub fn union_rect(r: Rect, other: Rect) -> Rect {
    Rect {
        x: union_interval(r.x, other.x),
        y: union_interval(r.y, other.y),
    }
}

pub fn union_interval(i: Interval, other: Interval) -> Interval {
    if i.is_empty() {
        other
    } else if other.is_empty() {
        i
    } else {
        Interval {
            lo: i.lo.min(other.lo),
            hi: i.hi.max(other.hi),
        }
    }
}

pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}
