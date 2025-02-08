

use std::f64;

const GEO_S1_EPSILON: f64 = 1e-15;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn approx_equal(&self, other: &GeoS1Interval) -> bool {
        if self.is_empty() {
            return other.length() <= 2.0 * GEO_S1_EPSILON;
        }
        if other.is_empty() {
            return self.length() <= 2.0 * GEO_S1_EPSILON;
        }
        if self.is_full() {
            return other.length() >= 2.0 * (f64::consts::PI - GEO_S1_EPSILON);
        }
        if other.is_full() {
            return self.length() >= 2.0 * (f64::consts::PI - GEO_S1_EPSILON);
        }

        (remainder(other.lo - self.lo, 2.0 * f64::consts::PI).abs() <= GEO_S1_EPSILON &&
        remainder(other.hi - self.hi, 2.0 * f64::consts::PI).abs() <= GEO_S1_EPSILON &&
        (self.length() - other.length()).abs() <= 2.0 * GEO_S1_EPSILON)
    }

    fn is_empty(&self) -> bool { self.lo == f64::consts::PI && self.hi == -f64::consts::PI }

    fn length(&self) -> f64 {
        let l = self.hi - self.lo;
        if l >= 0.0 {
            return l;
        }
        l + 2.0 * f64::consts::PI
    }

    fn is_full(&self) -> bool { self.lo == -f64::consts::PI && self.hi == f64::consts::PI }
}

fn remainder(x: f64, y: f64) -> f64 {
    x - (y * (x / y).floor())
}

fn main() {
    let i1 = GeoS1Interval { lo: 5.42695145753e-312, hi: 1.385239597708693e-309 };
    let i2 = GeoS1Interval { lo: 3.2379e-319, hi: -5.486124068793689e+303 };
    println!("{}", i1.approx_equal(&i2));
}

