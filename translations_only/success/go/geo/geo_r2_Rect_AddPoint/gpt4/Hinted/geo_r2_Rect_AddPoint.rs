
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Point {
    x: f64,
    y: f64,
}

impl GeoR2Rect {
    fn add_point(&self, p: GeoR2Point) -> Box<GeoR2Rect> {
        Box::new(GeoR2Rect {
            x: *add_point_interval(&self.x, p.x),
            y: *add_point_interval(&self.y, p.y),
        })
    }
}

fn add_point_interval(i: &GeoR1Interval, p: f64) -> Box<GeoR1Interval> {
    if is_empty_interval(i) {
        return Box::new(GeoR1Interval { lo: p, hi: p });
    }
    if p < i.lo {
        return Box::new(GeoR1Interval { lo: p, hi: i.hi });
    }
    if p > i.hi {
        return Box::new(GeoR1Interval { lo: i.lo, hi: p });
    }
    Box::new(GeoR1Interval { lo: i.lo, hi: i.hi })
}

fn is_empty_interval(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}
