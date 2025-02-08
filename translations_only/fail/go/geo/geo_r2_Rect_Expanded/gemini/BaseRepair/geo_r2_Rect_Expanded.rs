
pub fn expanded_rect(r: Rect, margin: Point) -> Rect {
    let xx = expanded_interval(r.x, margin.x);
    let yy = expanded_interval(r.y, margin.y);
    if is_empty_interval(xx) || is_empty_interval(yy) {
        return empty_rect();
    }
    Rect { x: xx, y: yy }
}

pub fn expanded_interval(i: Interval, margin: f64) -> Interval {
    if is_empty_interval(i) {
        return i;
    }
    Interval {
        lo: i.lo - margin,
        hi: i.hi + margin,
    }
}

pub fn is_empty_interval(i: Interval) -> bool {
    i.lo > i.hi
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

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
