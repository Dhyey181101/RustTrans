
fn interior_contains(r1: &Rect, r2: &Rect) -> bool {
    interior_contains_interval(&r1.x, &r2.x) && interior_contains_interval(&r1.y, &r2.y)
}

fn interior_contains_interval(i1: &Interval, i2: &Interval) -> bool {
    if i2.lo > i2.hi {
        return true;
    }
    i1.lo < i2.lo && i2.hi < i1.hi
}

fn is_empty(i: &Interval) -> bool {
    i.lo > i.hi
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x: Interval,
    y: Interval,
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    lo: f64,
    hi: f64,
}
