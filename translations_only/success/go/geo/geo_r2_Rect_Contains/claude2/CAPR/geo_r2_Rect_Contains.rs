

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn contains_interval(i: &GeoR1Interval, oi: &GeoR1Interval) -> bool {
    if is_empty(oi) {
        true
    } else {
        i.lo <= oi.lo && oi.hi <= i.hi
    }
}

fn contains(r: &GeoR2Rect, other: &GeoR2Rect) -> bool {
    contains_interval(&r.x, &other.x) && contains_interval(&r.y, &other.y)
}

