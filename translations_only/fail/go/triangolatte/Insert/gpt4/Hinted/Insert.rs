
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Point {
    x: f64,
    y: f64,
}

struct Element {
    prev: RefCell<Weak<RefCell<Element>>>,
    next: RefCell<Weak<RefCell<Element>>>,
    point: Point,
}

fn insert(p: Point, e: Option<Rc<RefCell<Element>>>) -> Rc<RefCell<Element>> {
    let new_element = Rc::new(RefCell::new(Element {
        prev: RefCell::new(Weak::new()),
        next: RefCell::new(Weak::new()),
        point: p,
    }));

    if let Some(existing_element) = e {
        let next = existing_element.borrow().next.borrow().upgrade().unwrap();
        *new_element.borrow_mut().next.borrow_mut() = Rc::downgrade(&next);
        *new_element.borrow_mut().prev.borrow_mut() = Rc::downgrade(&existing_element);
        *existing_element.borrow_mut().next.borrow_mut() = Rc::downgrade(&new_element);
        *next.borrow_mut().prev.borrow_mut() = Rc::downgrade(&new_element);
    } else {
        *new_element.borrow_mut().prev.borrow_mut() = Rc::downgrade(&new_element);
        *new_element.borrow_mut().next.borrow_mut() = Rc::downgrade(&new_element);
    }

    new_element
}

type ByMaxX = Vec<Vec<Point>>;
