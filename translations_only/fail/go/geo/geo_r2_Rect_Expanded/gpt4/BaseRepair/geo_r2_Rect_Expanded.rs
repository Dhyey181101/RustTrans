
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

fn expanded_rect(r: &GeoR2Rect, margin: &GeoR2Point) -> GeoR2Rect {
    let xx = expanded_interval(&r.x, margin.x);
    let yy = expanded_interval(&r.y, margin.y);
    if is_empty_interval(&xx) || is_empty_interval(&yy) {
        geo_r2_empty_rect()
    } else {
        GeoR2Rect { x: xx, y: yy }
    }
}

fn expanded_interval(i: &GeoR1Interval, margin: f64) -> GeoR1Interval {
    if is_empty_interval(i) {
        GeoR1Interval { lo: i.lo, hi: i.hi }
    } else {
        GeoR1Interval {
            lo: i.lo - margin,
            hi: i.hi + margin,
        }
    }
}

fn is_empty_interval(i: &GeoR1Interval) -> bool {
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
