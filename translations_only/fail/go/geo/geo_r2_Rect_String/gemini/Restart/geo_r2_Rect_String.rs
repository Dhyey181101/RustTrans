
use std::fmt;

#[derive(Debug)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

#[derive(Debug)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Lo{}, Hi{}]", self.lo(), self.hi())
    }
}

impl Rect {
    pub fn lo(&self) -> Point {
        Point {
            x: self.x.lo,
            y: self.y.lo,
        }
    }

    pub fn hi(&self) -> Point {
        Point {
            x: self.x.hi,
            y: self.y.hi,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
