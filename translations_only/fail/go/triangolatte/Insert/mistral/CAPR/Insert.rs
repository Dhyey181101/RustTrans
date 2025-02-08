

use std::boxed::Box;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Element {
    prev: RefCell<Option<Rc<Element>>>,
    next: RefCell<Option<Rc<Element>>>,
    point: Point,
}

pub struct Point {
    x: f64,
    y: f64,
}

pub type ByMaxX = Vec<Vec<Point>>;

pub fn insert(p: Point, e: &Option<Rc<Element>>) -> Rc<Element> {
    let new = Rc::new(Element {
        prev: RefCell::new(None),
        next: RefCell::new(None),
        point: p,
    });

    if let Some(elem) = e {
        *elem.next.borrow_mut() = Some(new.clone());
        *elem.prev.borrow_mut() = Some(new.clone());
        *elem.next.borrow_mut() = Some(Rc::clone(&new));
    } else {
        *new.prev.borrow_mut() = Some(new.clone());
        *new.next.borrow_mut() = Some(new.clone());
    }

    new
}

