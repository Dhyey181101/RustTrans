
pub fn add_point_rect(r: &Rect, p: &Point) -> Rect {
    Rect {
        x: add_point_interval(&r.x, &p.x),
        y: add_point_interval(&r.y, &p.y),
    }
}

pub fn add_point_interval(i: &Interval, p: &f64) -> Interval {
    if is_empty_interval(i) {
        Interval { lo: *p, hi: *p }
    } else if *p < i.lo {
        Interval { lo: *p, hi: i.hi }
    } else if *p > i.hi {
        Interval { lo: i.lo, hi: *p }
    } else {
        *i
    }
}

pub fn is_empty_interval(i: &Interval) -> bool {
    i.lo > i.hi
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

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

