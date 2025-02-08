
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

#[derive(Clone, Copy, Debug)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

#[derive(Clone, Copy, Debug)]
struct GeoR2Point {
    x: f64,
    y: f64,
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

fn geo_r2_rect_expanded_by_margin(r: GeoR2Rect, margin: f64) -> GeoR2Rect {
    geo_r2_rect_expanded(r, GeoR2Point { x: margin, y: margin })
}

fn geo_r2_rect_expanded(r: GeoR2Rect, margin: GeoR2Point) -> GeoR2Rect {
    let xx = geo_r1_interval_expanded(r.x, margin.x);
    let yy = geo_r1_interval_expanded(r.y, margin.y);
    if geo_r1_interval_is_empty(xx) || geo_r1_interval_is_empty(yy) {
        return geo_r2_empty_rect();
    }
    GeoR2Rect { x: xx, y: yy }
}

fn geo_r1_interval_expanded(i: GeoR1Interval, margin: f64) -> GeoR1Interval {
    if geo_r1_interval_is_empty(i) {
        return i;
    }
    GeoR1Interval {
        lo: i.lo - margin,
        hi: i.hi + margin,
    }
}

fn geo_r1_interval_is_empty(i: GeoR1Interval) -> bool {
    i.lo > i.hi
}
