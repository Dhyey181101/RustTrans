
use std::fmt;

#[derive(Debug, Clone, Copy)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn lo(&self) -> f64 {
        self.lo
    }

    fn hi(&self) -> f64 {
        self.hi
    }
}

#[derive(Debug, Clone, Copy)]
struct GeoR2Point {
    x: f64,
    y: f64,
}

impl GeoR2Point {
    fn new(x: f64, y: f64) -> Self {
        GeoR2Point { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

impl GeoR2Rect {
    fn lo(&self) -> GeoR2Point {
        GeoR2Point::new(self.x.lo(), self.y.lo())
    }

    fn hi(&self) -> GeoR2Point {
        GeoR2Point::new(self.x.hi(), self.y.hi())
    }
}

impl fmt::Display for GeoR2Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Lo{{{:?}, {:?}}}, Hi{{{:?}, {:?}}}]",
            self.lo().x, self.lo().y, self.hi().x, self.hi().y
        )
    }
}
