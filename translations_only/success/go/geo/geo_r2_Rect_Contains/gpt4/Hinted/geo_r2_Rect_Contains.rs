
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn new(lo: f64, hi: f64) -> Self {
        GeoR1Interval { lo, hi }
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

fn contains_interval(i: &GeoR1Interval, oi: &GeoR1Interval) -> bool {
    if oi.is_empty() {
        return true;
    }
    i.lo <= oi.lo && oi.hi <= i.hi
}

fn contains(r: &GeoR2Rect, other: &GeoR2Rect) -> bool {
    contains_interval(&r.x, &other.x) && contains_interval(&r.y, &other.y)
}

fn main() {}
