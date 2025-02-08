
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

    fn scale(&self, f: f64) -> PointBox {
        let Point { x, y } = *self.0;
        let norm = f64::sqrt(x * x + y * y);
        PointBox(Box::new(Point {
            x: x / norm * f,
            y: y / norm * f,
        }))
    }
}
