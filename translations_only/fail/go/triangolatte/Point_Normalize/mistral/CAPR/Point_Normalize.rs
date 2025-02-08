
use std::f64;
use std::ops::Mul;

fn normalize(p: Point) -> Point {
    let norm = (p.x * p.x + p.y * p.y).sqrt();
    Point {
        x: p.x / norm,
        y: p.y / norm,
    }
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x * p.x + p.y * p.y).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, f: f64) -> Point {
        Point {
            x: self.x * f,
            y: self.y * f,
        }
    }
}
