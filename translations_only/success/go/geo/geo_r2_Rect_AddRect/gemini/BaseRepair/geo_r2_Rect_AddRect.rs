
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

impl geo_r2_Rect {
    pub fn add_rect(&self, other: geo_r2_Rect) -> geo_r2_Rect {
        geo_r2_Rect {
            x: self.x.union(other.x),
            y: self.y.union(other.y),
        }
    }
}

impl geo_r1_Interval {
    pub fn union(&self, other: geo_r1_Interval) -> geo_r1_Interval {
        if self.is_empty() {
            other
        } else if other.is_empty() {
            *self
        } else {
            geo_r1_Interval {
                lo: f64::min(self.lo, other.lo),
                hi: f64::max(self.hi, other.hi),
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}
