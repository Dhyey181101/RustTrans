
use std::boxed::Box;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
    point: Point,
}

fn insert(p: Point, e: Option<Box<Element>>) -> Option<Box<Element>> {
    let mut new = Box::new(Element {
        prev: None,
        next: None,
        point: p,
    });

    if let Some(mut e) = e {
        new.next = e.next.clone();
        new.prev = Some(e.clone());
        if let Some(next) = e.next.as_mut() {
            next.prev = Some(new.clone());
        }
        e.next = Some(new.clone());
    } else {
        let new_clone = new.clone();
        new.prev = Some(new_clone.clone());
        new.next = Some(new_clone);
    }
    Some(new)
}

type ByMaxX = Vec<Vec<Point>>;
