

use std::f64;

struct GeoS1Interval {
lo: f64,
hi: f64,
}

impl GeoS1Interval {
fn new(lo: f64, hi: f64) -> GeoS1Interval {
GeoS1Interval { lo, hi }
}

fn complement(&self) -> GeoS1Interval {
if self.lo == self.hi {
GeoS1Interval::full_interval()
} else {
GeoS1Interval::new(self.hi, self.lo)
}
}

fn center(&self) -> f64 {
0.5 * (self.lo + self.hi)
}

fn full_interval() -> GeoS1Interval {
GeoS1Interval {
lo: -f64::consts::PI,
hi: f64::consts::PI,
}
}

fn is_inverted(&self) -> bool {
self.lo > self.hi
}
}

fn complement_center(i: &GeoS1Interval) -> f64 {
if !i.is_inverted() {
let complemented = i.complement();
complemented.center()
} else {
i.center()
}
}

fn center(i: &GeoS1Interval) -> f64 {
0.5 * (i.lo + i.hi)
}

fn geo_s1_full_interval() -> GeoS1Interval {
GeoS1Interval::full_interval()
}

