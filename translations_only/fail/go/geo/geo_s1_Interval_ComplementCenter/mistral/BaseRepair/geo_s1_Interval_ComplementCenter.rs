

use std::f64;

fn complement_center(i: &geo_s1_Interval) -> f64 {
    if i.lo != i.hi {
        let complemented = complement(i);
        center(&complemented)
    } else {
        // Singleton. The interval just contains a single point.
        if i.hi <= 0.0 {
            i.hi + f64::consts::PI
        } else {
            i.hi - f64::consts::PI
        }
    }
}

fn complement(i: &geo_s1_Interval) -> Box<geo_s1_Interval> {
    if i.lo == i.hi {
        // Singleton. The interval just contains a single point.
        Box::new(geo_s1_full_interval())
    } else {
        // Handles empty and full.
        Box::new(geo_s1_Interval {
            lo: i.hi,
            hi: i.lo,
        })
    }
}

fn geo_s1_full_interval() -> geo_s1_Interval {
    geo_s1_Interval {
        lo: -f64::consts::PI,
        hi: f64::consts::PI,
    }
}

fn center(i: &geo_s1_Interval) -> f64 {
    0.5 * (i.lo + i.hi)
}

fn is_inverted(i: &geo_s1_Interval) -> bool {
    i.lo > i.hi
}

#[derive(Debug, Clone, Copy)]
struct geo_s1_Interval {
    lo: f64,
    hi: f64,
}

impl geo_s1_Interval {
    fn center(&self) -> f64 {
        0.5 * (self.lo + self.hi)
    }
}

