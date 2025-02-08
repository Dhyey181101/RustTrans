
struct Interval {
    lo: f64,
    hi: f64,
}

impl Interval {
    fn contains_interval(&self, oi: &Interval) -> bool {
        if oi.is_empty() {
            return true;
        }
        self.lo <= oi.lo && oi.hi <= self.hi
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

struct Rect {
    x: Box<Interval>,
    y: Box<Interval>,
}

impl Rect {
    fn contains(&self, other: &Rect) -> bool {
        self.x.contains_interval(&other.x) && self.y.contains_interval(&other.y)
    }
}
