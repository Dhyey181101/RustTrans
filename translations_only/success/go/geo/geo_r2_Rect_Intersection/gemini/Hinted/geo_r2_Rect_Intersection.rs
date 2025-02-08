
use std::f64::consts::PI;

pub fn rect_intersection(r: Rect, other: Rect) -> Rect {
    let xx = interval_intersection(r.x, other.x);
    let yy = interval_intersection(r.y, other.y);
    if xx.lo > xx.hi || yy.lo > yy.hi {
        return empty_rect();
    }

    Rect { x: xx, y: yy }
}

pub fn interval_intersection(i: Interval, j: Interval) -> Interval {
    Interval {
        lo: i.lo.max(j.lo),
        hi: i.hi.min(j.hi),
    }
}

pub fn empty_rect() -> Rect {
    Rect {
        x: empty_interval(),
        y: empty_interval(),
    }
}

pub fn empty_interval() -> Interval {
    Interval { lo: 1.0, hi: 0.0 }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}
