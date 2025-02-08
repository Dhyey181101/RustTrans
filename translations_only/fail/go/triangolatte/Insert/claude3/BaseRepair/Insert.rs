
#[derive(Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]
pub struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
    point: Point,
}

pub fn insert(p: Point, e: Option<Box<Element>>) -> Box<Element> {
    let mut new = Box::new(Element {
        prev: None,
        next: None,
        point: p,
    });

    if let Some(mut e) = e {
        new.next = e.next.clone();
        new.prev = Some(e.clone());
        if let Some(ref mut next) = e.next {
            next.prev = Some(new.clone());
        }
        e.next = Some(new.clone());
    } else {
        new.prev = Some(new.clone());
        new.next = Some(new.clone());
    }

    new
}
