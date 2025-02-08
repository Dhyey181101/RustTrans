
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR2Rect {
    fn add_rect(&self, other: GeoR2Rect) -> GeoR2Rect {
        GeoR2Rect {
            x: union_interval(&self.x, &other.x),
            y: union_interval(&self.y, &other.y),
        }
    }
}

fn union_interval(i: &GeoR1Interval, other: &GeoR1Interval) -> GeoR1Interval {
    if is_empty_interval(i) {
        return GeoR1Interval { lo: other.lo, hi: other.hi };
    }
    if is_empty_interval(other) {
        return GeoR1Interval { lo: i.lo, hi: i.hi };
    }
    GeoR1Interval {
        lo: i.lo.min(other.lo),
        hi: i.hi.max(other.hi),
    }
}

fn is_empty_interval(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}
