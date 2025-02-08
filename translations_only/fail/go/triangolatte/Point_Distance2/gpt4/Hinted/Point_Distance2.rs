
use std::f64;

struct Point {
    X: f64,
    Y: f64,
}

fn distance2(p: Box<Point>, r: Box<Point>) -> f64 {
    (p.X - r.X) * (p.X - r.X) + (p.Y - r.Y) * (p.Y - r.Y)
}

fn main() {}
