
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

fn interior_contains_interval(i: &GeoR1Interval, oi: &GeoR1Interval) -> bool {
    if oi.is_empty() {
        return true;
    }
    i.lo < oi.lo && oi.hi < i.hi
}

fn interior_contains(r: &GeoR2Rect, other: &GeoR2Rect) -> bool {
    interior_contains_interval(&r.x, &other.x) && interior_contains_interval(&r.y, &other.y)
}
