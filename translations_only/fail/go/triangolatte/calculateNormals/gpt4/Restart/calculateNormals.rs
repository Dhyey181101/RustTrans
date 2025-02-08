
use std::f64;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn normalize(&self) -> Box<Point> {
        self.scale(1.0)
    }

    fn scale(&self, f: f64) -> Box<Point> {
        let norm = f64::sqrt(self.x * self.x + self.y * self.y);
        Box::new(Point {
            x: self.x / norm * f,
            y: self.y / norm * f,
        })
    }
}

fn calculate_normals(x: f64, y: f64) -> [Box<Point>; 2] {
    [
        Point { x: y, y: -x }.normalize(),
        Point { x: -y, y: x }.normalize(),
    ]
}

fn main() {}
