
use std::f64;
use std::boxed;

struct Point {
    x: f64,
    y: f64,
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        Point::new(y, -x).normalize(),
        Point::new(-y, x).normalize(),
    ]
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn normalize(mut self) -> Point {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        Point {
            x: self.x / norm,
            y: self.y / norm,
        }
    }

    fn scale(self, f: f64) -> Point {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        Point {
            x: self.x / norm * f,
            y: self.y / norm * f,
        }
    }
}
