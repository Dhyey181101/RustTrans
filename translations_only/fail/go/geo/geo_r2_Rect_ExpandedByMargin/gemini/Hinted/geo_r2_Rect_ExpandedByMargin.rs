
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
    pub fn expanded_by_margin(&self, margin: f64) -> Rect {
        self.expanded(Point { x: margin, y: margin })
    }

    pub fn expanded(&self, margin: Point) -> Rect {
        let xx = self.x.expanded(margin.x);
        let yy = self.y.expanded(margin.y);
        if xx.is_empty() || yy.is_empty() {
            Rect::empty()
        } else {
            Rect { x: xx, y: yy }
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

impl Rect {
    pub fn empty() -> Rect {
        Rect {
            x: Interval::empty(),
            y: Interval::empty(),
        }
    }
}

impl Interval {
    pub fn empty() -> Interval {
        Interval { lo: f64::INFINITY, hi: 0.0 }
    }
}
