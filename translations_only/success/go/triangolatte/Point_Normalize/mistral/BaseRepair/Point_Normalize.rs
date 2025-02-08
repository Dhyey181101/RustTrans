
use std::f64;
use std::ops::Mul;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn normalize(&self) -> Point {
        self.scale(1.0)
    }

    fn scale(&self, f: f64) -> Point {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        Point {
            x: (self.x / norm) * f,
            y: (self.y / norm) * f,
        }
    }
}
