
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR2Rect {
    fn union(&self, other: &GeoR2Rect) -> GeoR2Rect {
        GeoR2Rect {
            x: union_r1(&self.x, &other.x),
            y: union_r1(&self.y, &other.y),
        }
    }
}

fn union_r1(self_interval: &GeoR1Interval, other_interval: &GeoR1Interval) -> GeoR1Interval {
    if is_empty_r1(self_interval) {
        return GeoR1Interval {
            lo: other_interval.lo,
            hi: other_interval.hi,
        };
    }
    if is_empty_r1(other_interval) {
        return GeoR1Interval {
            lo: self_interval.lo,
            hi: self_interval.hi,
        };
    }
    GeoR1Interval {
        lo: self_interval.lo.min(other_interval.lo),
        hi: self_interval.hi.max(other_interval.hi),
    }
}

fn is_empty_r1(interval: &GeoR1Interval) -> bool {
    interval.lo > interval.hi
}
