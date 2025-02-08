
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

    if let Some(mut elem) = e {
        let new_clone = new.clone();
        new.next = elem.next.clone();
        new.prev = Some(elem.clone());
        if let Some(next) = new.next.as_mut() {
            next.prev = Some(new_clone);
        }
        elem.next = Some(new.clone());
    } else {
        let new_clone = new.clone();
        new.prev = Some(new_clone.clone());
        new.next = Some(new_clone);
    }

    new
}
