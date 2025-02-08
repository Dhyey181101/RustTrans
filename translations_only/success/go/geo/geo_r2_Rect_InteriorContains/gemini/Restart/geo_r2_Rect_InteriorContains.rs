
pub fn interior_contains_interval(i: &Interval, oi: &Interval) -> bool {
    if oi.is_empty() {
        return true;
    }
    i.lo < oi.lo && oi.hi < i.hi
}

pub fn interior_contains(r: &Rect, other: &Rect) -> bool {
    interior_contains_interval(&r.x, &other.x) && interior_contains_interval(&r.y, &other.y)
}

pub fn is_empty(i: &Interval) -> bool {
    i.lo > i.hi
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn is_empty(&self) -> bool {
        is_empty(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

impl Rect {
    pub fn interior_contains(&self, other: &Rect) -> bool {
        interior_contains(self, other)
    }
}
