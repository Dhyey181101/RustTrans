
use std::f64::consts::PI;

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        Point {
            x: y,
            y: -x,
        }
        .normalize(),
        Point {
            x: -y,
            y: x,
        }
        .normalize(),
    ]
}

impl Point {
    fn normalize(&self) -> Point {
        self.scale(1.0)
    }

    fn scale(&self, f: f64) -> Point {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        Point {
            x: self.x / norm * f,
            y: self.y / norm * f,
        }
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}
