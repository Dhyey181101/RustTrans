
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

fn normalize(p: Point) -> Point {
    let norm = (p.x.powi(2) + p.y.powi(2)).sqrt();
    Point {
        x: p.x / norm,
        y: p.y / norm
    }
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x.powi(2) + p.y.powi(2)).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f
    }
}

fn add(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y
    }
}

fn sub(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x - p2.x,
        y: p1.y - p2.y
    }
}

fn dot(p1: Point, p2: Point) -> f64 {
    p1.x * p2.x + p1.y * p2.y
}

fn miter(points: &[Point], width: f64) -> Vec<f64> {
    vec![]
}

