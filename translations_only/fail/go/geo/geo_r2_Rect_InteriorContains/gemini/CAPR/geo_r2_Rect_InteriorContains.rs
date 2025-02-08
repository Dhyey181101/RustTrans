
use std::cmp::{max, min};

pub fn interior_contains_interval(i: &Interval, oi: &Interval) -> bool {
    if oi.lo >= i.lo && oi.hi <= i.hi {
        return true;
    }
    false
}

pub fn is_empty(i: &Interval) -> bool {
    i.lo > i.hi
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

impl Rect {
    pub fn interior_contains(&self, other: &Rect) -> bool {
        interior_contains_interval(&self.x, &other.x) && interior_contains_interval(&self.y, &other.y)
    }
}

