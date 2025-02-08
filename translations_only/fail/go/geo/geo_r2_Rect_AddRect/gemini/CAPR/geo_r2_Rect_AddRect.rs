
use std::f64::consts::PI;

pub fn add_rect(r: geo_r2_Rect, other: geo_r2_Rect) -> geo_r2_Rect {
    geo_r2_Rect {
        X: union(r.X, other.X),
        Y: union(r.Y, other.Y),
    }
}

pub fn union(i: geo_r1_Interval, other: geo_r1_Interval) -> geo_r1_Interval {
    if i.Lo > i.Hi {
        other
    } else if other.Lo > other.Hi {
        i
    } else {
        geo_r1_Interval {
            Lo: i.Lo.min(other.Lo),
            Hi: i.Hi.max(other.Hi),
        }
    }
}

pub struct geo_r2_Rect {
    pub X: geo_r1_Interval,
    pub Y: geo_r1_Interval,
}

pub struct geo_r1_Interval {
    pub Lo: f64,
    pub Hi: f64,
}
