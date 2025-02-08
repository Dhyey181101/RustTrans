
struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn contains_interval(&self, oi: &GeoS1Interval) -> bool {
        if self.is_inverted() {
            if oi.is_inverted() {
                return oi.lo >= self.lo && oi.hi <= self.hi;
            }
            return (oi.lo >= self.lo || oi.hi <= self.hi) && !self.is_empty();
        }
        if oi.is_inverted() {
            return self.is_full() || oi.is_empty();
        }
        return oi.lo >= self.lo && oi.hi <= self.hi;
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_empty(&self) -> bool {
        self.lo == std::f64::consts::PI && self.hi == -std::f64::consts::PI
    }

    fn is_full(&self) -> bool {
        self.lo == -std::f64::consts::PI && self.hi == std::f64::consts::PI
    }
}
