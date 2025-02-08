
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

fn intersection_r2(r: &GeoR2Rect, other: &GeoR2Rect) -> GeoR2Rect {
    let xx = intersection_r1(&r.x, &other.x);
    let yy = intersection_r1(&r.y, &other.y);
    if is_empty_r1(&xx) || is_empty_r1(&yy) {
        return geo_r2_empty_rect();
    }

    GeoR2Rect { x: xx, y: yy }
}

fn intersection_r1(i: &GeoR1Interval, j: &GeoR1Interval) -> GeoR1Interval {
    GeoR1Interval {
        lo: i.lo.max(j.lo),
        hi: i.hi.min(j.hi),
    }
}

fn is_empty_r1(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn geo_r2_empty_rect() -> GeoR2Rect {
    GeoR2Rect {
        x: geo_r1_empty_interval(),
        y: geo_r1_empty_interval(),
    }
}

fn geo_r1_empty_interval() -> GeoR1Interval {
    GeoR1Interval { lo: 1.0, hi: 0.0 }
}
