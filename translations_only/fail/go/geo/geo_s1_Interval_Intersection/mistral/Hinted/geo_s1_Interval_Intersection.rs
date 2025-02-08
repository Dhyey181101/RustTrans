

use std::f64;

#[derive(Clone)]
struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn new(lo: f64, hi: f64) -> Self {
        GeoS1Interval { lo, hi }
    }

    fn intersection(&self, oi: &GeoS1Interval) -> GeoS1Interval {
        if oi.is_empty() {
            return GeoS1Interval::new(f64::consts::PI, -f64::consts::PI);
        }
        if self.fast_contains(oi.lo) {
            if self.fast_contains(oi.hi) {
                if oi.length() < self.length() {
                    return oi.to_owned()
                }
                return self.to_owned()
            }
            return GeoS1Interval::new(oi.lo, self.hi);
        }
        if self.fast_contains(oi.hi) {
            return GeoS1Interval::new(self.lo, oi.hi);
        }
        if oi.fast_contains(self.lo) {
            return self.to_owned();
        }
        GeoS1Interval::new(f64::consts::PI, -f64::consts::PI)
    }

    fn is_empty(&self) -> bool {
        self.lo == f64::consts::PI && self.hi == -f64::consts::PI
    }

    fn fast_contains(&self, p: f64) -> bool {
        if self.is_inverted() {
            return p >= self.lo || p <= self.hi && !self.is_empty();
        }
        p >= self.lo && p <= self.hi
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn length(&self) -> f64 {
        let l = self.hi - self.lo;
        if l >= 0.0 {
            return l;
        }
        l + 2.0 * f64::consts::PI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let i1 = GeoS1Interval::new(2.305009411495597e-76, 3.1325804091391376e-294);
        let i2 = GeoS1Interval::new(2.774480017624387e+180, -3.690181449604242e-74);
        assert_eq!(i2, i1.intersection(&i2));

        let i1 = GeoS1Interval::new(0.0, 4.778309726736743e-299);
        let i2 = GeoS1Interval::new(5.06e-321, 1.0655986769484148e-255);
        assert_eq!(i1, i2.intersection(&i1));

        let i1 = GeoS1Interval::new(7.414152788647802e+144, -3.1033252149811696e-215);
        let i2 = GeoS1Interval::new(2.744088057891335e+180, 3.8496525446028015e+178);
        assert_eq!(i1, i2.intersection(&i1));

        let i1 = GeoS1Interval::new(7.41415280700048e+144, 1.0477123566e-314);
        let i2 = GeoS1Interval::new(1.971020260030814e+181, 0.0);
        assert_eq!(i1, i2.intersection(&i1));
    }
}

