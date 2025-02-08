
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

impl Rect {
    pub fn intersection(r: Rect, other: Rect) -> Rect {
        let xx = intersection(r.x, other.x);
        let yy = intersection(r.y, other.y);
        if xx.is_empty() || yy.is_empty() {
            empty_rect()
        } else {
            Rect { x: xx, y: yy }
        }
    }
}

fn intersection(i: Interval, j: Interval) -> Interval {
    Interval {
        lo: f64::max(i.lo, j.lo),
        hi: f64::min(i.hi, j.hi),
    }
}

fn empty_rect() -> Rect {
    Rect {
        x: empty_interval(),
        y: empty_interval(),
    }
}

fn empty_interval() -> Interval {
    Interval { lo: 1.0, hi: 0.0 }
}
