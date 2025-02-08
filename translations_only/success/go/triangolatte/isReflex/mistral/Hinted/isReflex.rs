

use std::boxed::Box;

pub struct Point {
    x: f64,
    y: f64,
}

pub fn is_reflex(a: Box<Point>, b: Box<Point>, c: Box<Point>) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

