
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
    pub fn add_point(self, p: Point) -> Rect {
        Rect {
            x: self.x.add_point(p.x),
            y: self.y.add_point(p.y),
        }
    }
}

impl Interval {
    pub fn add_point(self, p: f64) -> Interval {
        if self.is_empty() {
            Interval { lo: p, hi: p }
        } else if p < self.lo {
            Interval { lo: p, hi: self.hi }
        } else if p > self.hi {
            Interval { lo: self.lo, hi: p }
        } else {
            self
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}
