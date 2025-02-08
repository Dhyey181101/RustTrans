
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn scale(&self, f: f64) -> Point {
        let norm = (self.x.powi(2) + self.y.powi(2)).sqrt();
        Point {
            x: self.x / norm * f,
            y: self.y / norm * f,
        }
    }
}
