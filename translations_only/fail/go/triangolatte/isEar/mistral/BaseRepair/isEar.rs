
use std::boxed::Box;

struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
    point: Point,
}

struct Point {
    x: f64,
    y: f64,
}
