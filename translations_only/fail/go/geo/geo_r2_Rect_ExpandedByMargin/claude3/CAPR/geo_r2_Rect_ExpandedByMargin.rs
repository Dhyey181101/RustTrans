
use std::ops::Range;

#[derive(Debug, Clone, Copy)]
struct Geo_r2_Rect {
    x: Geo_r1_Interval,
    y: Geo_r1_Interval,
}

#[derive(Debug, Clone, Copy)]
struct Geo_r1_Interval {
    lo: f64,
    hi: f64,
}

#[derive(Debug, Clone, Copy)]
struct Geo_r2_Point {
    x: f64,
    y: f64,
}

fn geo_r2_expanded_by_margin(r: Geo_r2_Rect, margin: f64) -> Geo_r2_Rect {
    geo_r2_expanded(r, Geo_r2_Point { x: margin, y: margin })
}

fn geo_r2_expanded(r: Geo_r2_Rect, margin: Geo_r2_Point) -> Geo_r2_Rect {
    let xx = geo_r1_expanded(r.x, margin.x);
    let yy = geo_r1_expanded(r.y, margin.y);
    if geo_r1_is_empty(xx) || geo_r1_is_empty(yy) {
        return geo_r2_empty_rect();
    }
    Geo_r2_Rect { x: xx, y: yy }
}

fn geo_r1_expanded(i: Geo_r1_Interval, margin: f64) -> Geo_r1_Interval {
    if geo_r1_is_empty(i) {
        return i;
    }
    Geo_r1_Interval {
        lo: i.lo - margin,
        hi: i.hi + margin,
    }
}

fn geo_r1_is_empty(i: Geo_r1_Interval) -> bool {
    i.lo > i.hi
}

fn geo_r2_empty_rect() -> Geo_r2_Rect {
    Geo_r2_Rect {
        x: geo_r1_empty_interval(),
        y: geo_r1_empty_interval(),
    }
}

fn geo_r1_empty_interval() -> Geo_r1_Interval {
    Geo_r1_Interval { lo: 1.0, hi: 0.0 }
}
