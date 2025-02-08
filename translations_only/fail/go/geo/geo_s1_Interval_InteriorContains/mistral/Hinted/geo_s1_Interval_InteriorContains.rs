

use std::f64;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn interior_contains(i: &GeoS1Interval, p: &mut f64) -> bool {
    if *p == -f64::consts::PI {
        *p = f64::consts::PI;
    }
    if i.is_inverted() {
        return *p > i.lo || *p < i.hi;
    }
    (*p > i.lo && *p < i.hi) || i.is_full()
}

impl GeoS1Interval {
    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_full(&self) -> bool {
        self.lo == -f64::consts::PI && self.hi == f64::consts::PI
    }
}

fn main() {
    let mut p = 5.957005972021e-310;
    let i1 = GeoS1Interval { lo: -8.010577201524042e-113, hi: -7.796560589967655e-113 };
    assert!(!interior_contains(&i1, &mut p));

    let mut p = 8.188271799314787e-306;
    let i2 = GeoS1Interval { lo: 0.0, hi: -3.5514267919380624e-195 };
    assert!(interior_contains(&i2, &mut p));

    let mut p = 6.835426893334122e-304;
    let i3 = GeoS1Interval { lo: 1.30375421396906e-310, hi: -4.778309726736481e-299 };
    assert!(!interior_contains(&i3, &mut p));

    let mut p = -2.751642053659518e-135;
    let i4 = GeoS1Interval { lo: 5.092789904e-313, hi: 0.0 };
    assert!(!interior_contains(&i4, &mut p));
}
