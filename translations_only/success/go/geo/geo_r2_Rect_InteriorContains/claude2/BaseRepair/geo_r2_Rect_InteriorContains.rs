
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn interior_contains_interval(i: &GeoR1Interval, oi: &GeoR1Interval) -> bool {
    if is_empty(oi) {
        true
    } else {
        i.lo < oi.lo && oi.hi < i.hi
    }
}

fn is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn interior_contains(r: &Box<GeoR2Rect>, other: &Box<GeoR2Rect>) -> bool {
    interior_contains_interval(&r.x, &other.x) && interior_contains_interval(&r.y, &other.y)
}
