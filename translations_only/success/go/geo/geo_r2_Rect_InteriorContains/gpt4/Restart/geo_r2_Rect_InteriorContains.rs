
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR2Rect {
    fn interior_contains(&self, other: &GeoR2Rect) -> bool {
        interior_contains_interval(&self.x, &other.x) && interior_contains_interval(&self.y, &other.y)
    }
}

fn interior_contains_interval(i: &GeoR1Interval, oi: &GeoR1Interval) -> bool {
    if is_empty(oi) {
        return true;
    }
    i.lo < oi.lo && oi.hi < i.hi
}

fn is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn main() {}
