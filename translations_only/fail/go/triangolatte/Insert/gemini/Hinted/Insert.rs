
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

struct Element {
    prev: Option<Rc<RefCell<Element>>>,
    next: Option<Rc<RefCell<Element>>>,
    point: Point,
}

fn insert(p: Point, e: Option<Rc<RefCell<Element>>>) -> Rc<RefCell<Element>> {
    let new = Rc::new(RefCell::new(Element {
        prev: None,
        next: None,
        point: p,
    }));

    if let Some(e) = e {
        new.borrow_mut().next = e.borrow().next.clone();
        new.borrow_mut().prev = Some(e.clone());
        e.borrow_mut().next = Some(new.clone());
        if let Some(next) = new.borrow().next.clone() {
            next.borrow_mut().prev = Some(new.clone());
        }
    } else {
        new.borrow_mut().prev = Some(new.clone());
        new.borrow_mut().next = Some(new.clone());
    }

    new
}
