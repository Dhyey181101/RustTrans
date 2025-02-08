
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    pub fn add_point(mut self, p: f64) -> Interval {
        if self.is_empty() {
            self.lo = p;
            self.hi = p;
        } else if p < self.lo {
            self.lo = p;
        } else if p > self.hi {
            self.hi = p;
        }
        self
    }
}

impl Add for Interval {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Interval {
        Interval {
            lo: self.lo + rhs.lo,
            hi: self.hi + rhs.hi,
        }
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Interval) {
        self.lo += rhs.lo;
        self.hi += rhs.hi;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

impl Rect {
    pub fn add_point(self, p: Point) -> Rect {
        Rect {
            x: self.x.add_point(p.x),
            y: self.y.add_point(p.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
