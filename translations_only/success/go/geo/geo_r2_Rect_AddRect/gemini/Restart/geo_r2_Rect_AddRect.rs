
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;

pub fn add_rect(r: Rect, other: Rect) -> Rect {
    Rect {
        x: r.x.union(other.x),
        y: r.y.union(other.y),
    }
}

pub fn union(i: Interval, other: Interval) -> Interval {
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

pub fn is_empty(i: Interval) -> bool {
    i.lo > i.hi
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    pub fn union(&self, other: Interval) -> Interval {
        if self.is_empty() {
            other
        } else if other.is_empty() {
            *self
        } else {
            Interval {
                lo: self.lo.min(other.lo),
                hi: self.hi.max(other.hi),
            }
        }
    }
}

impl Rect {
    pub fn add_rect(&self, other: Rect) -> Rect {
        Rect {
            x: self.x.union(other.x),
            y: self.y.union(other.y),
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            lo: NEG_INFINITY,
            hi: INFINITY,
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            x: Interval::default(),
            y: Interval::default(),
        }
    }
}
