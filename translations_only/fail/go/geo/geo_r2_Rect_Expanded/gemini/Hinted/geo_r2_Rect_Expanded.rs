
use std::f64;

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

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Rect {
    pub fn expanded(&self, margin: Point) -> Rect {
        Rect {
            x: self.x.expanded(margin.x),
            y: self.y.expanded(margin.y),
        }
    }
}

impl Interval {
    pub fn expanded(&self, margin: f64) -> Interval {
        if self.is_empty() {
            *self
        } else {
            Interval {
                lo: self.lo - margin,
                hi: self.hi + margin,
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
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
