
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

fn union_r2_rect(r: Box<GeoR2Rect>, other: Box<GeoR2Rect>) -> Box<GeoR2Rect> {
    Box::new(GeoR2Rect {
        x: *union_r1_interval(Box::new(r.x), Box::new(other.x)),
        y: *union_r1_interval(Box::new(r.y), Box::new(other.y)),
    })
}

fn union_r1_interval(i: Box<GeoR1Interval>, other: Box<GeoR1Interval>) -> Box<GeoR1Interval> {
    if i.is_empty() {
        return other;
    }
    if other.is_empty() {
        return i;
    }
    Box::new(GeoR1Interval {
        lo: i.lo.min(other.lo),
        hi: i.hi.max(other.hi),
    })
}
