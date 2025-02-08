

use std::f64::consts::PI;

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct Element {
    prev: Box<Element>,
    next: Box<Element>,
    point: Point,
}

fn new_element() -> Element {
    Element {
        prev: Box::new(default_element()),
        next: Box::new(default_element()),
        point: Point { x: 0.0, y: 0.0 }
    }
}

fn default_element() -> Element {
    Element {
        prev: Box::new(default_element()),
        next: Box::new(default_element()),
        point: Point { x: 0.0, y: 0.0 }
    }
}

fn polygon(points: Vec<Point>) -> Result<Vec<f64>, String> {
    Ok(vec![])
}

