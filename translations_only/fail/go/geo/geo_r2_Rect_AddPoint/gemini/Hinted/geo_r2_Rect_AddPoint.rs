
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct geo_r2_Rect {
    pub x: geo_r1_Interval,
    pub y: geo_r1_Interval,
}

#[derive(Debug, Clone, Copy)]
pub struct geo_r1_Interval {
    pub lo: f64,
    pub hi: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct geo_r2_Point {
    pub x: f64,
    pub y: f64,
}

impl geo_r2_Rect {
    pub fn add_point(&self, p: geo_r2_Point) -> geo_r2_Rect {
        geo_r2_Rect {
            x: self.x.add_point(p.x),
            y: self.y.add_point(p.y),
        }
    }
}

impl geo_r1_Interval {
    pub fn add_point(&self, p: f64) -> geo_r1_Interval {
        if self.is_empty() {
            geo_r1_Interval { lo: p, hi: p }
        } else if p < self.lo {
            geo_r1_Interval { lo: p, hi: self.hi }
        } else if p > self.hi {
            geo_r1_Interval { lo: self.lo, hi: p }
        } else {
            *self
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}
