
struct Rect {
    x: Interval,
    y: Interval,
}

struct Interval {
    lo: f64,
    hi: f64,
}

impl Interval {
    fn contains_interval(&self, other: &Interval) -> bool {
        if other.is_empty() {
            return true;
        }
        self.lo <= other.lo && other.hi <= self.hi
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

fn rect_contains(r: Box<Rect>, other: Box<Rect>) -> bool {
    r.x.contains_interval(&other.x) && r.y.contains_interval(&other.y)
}
