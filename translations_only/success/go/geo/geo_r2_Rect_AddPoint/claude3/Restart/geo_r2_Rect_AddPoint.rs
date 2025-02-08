
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

fn add_point_rect(r: Box<GeoR2Rect>, p: GeoR2Point) -> Box<GeoR2Rect> {
    Box::new(GeoR2Rect {
        x: add_point_interval(Box::new(r.x), p.x),
        y: add_point_interval(Box::new(r.y), p.y),
    })
}

fn add_point_interval(i: Box<GeoR1Interval>, p: f64) -> GeoR1Interval {
    if is_empty_interval(&i) {
        return GeoR1Interval { lo: p, hi: p };
    }
    if p < i.lo {
        return GeoR1Interval { lo: p, hi: i.hi };
    }
    if p > i.hi {
        return GeoR1Interval { lo: i.lo, hi: p };
    }
    *i
}

fn is_empty_interval(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}
