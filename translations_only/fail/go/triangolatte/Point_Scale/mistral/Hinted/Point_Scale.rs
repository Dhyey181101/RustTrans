
use std::f64;

struct Point {
    x: f64,
    y: f64,
}

struct PointBox(Box<Point>);

impl PointBox {
    fn new(x: f64, y: f64) -> PointBox {
        PointBox(Box::new(Point { x, y }))
    }
}

impl Point {
    fn scale(&self, f: f64) -> Point {
        let norm = f64::sqrt(self.x * self.x + self.y * self.y);
        Point {
            x: self.x / norm * f,
            y: self.y / norm * f,
        }
    }
}

fn main() {
    let p = PointBox::new(3.0, 4.0);
    println!("Scale Point: {}", p.0.scale(1.5).x);
}
