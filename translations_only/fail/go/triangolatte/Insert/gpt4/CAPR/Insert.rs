
use std::rc::Rc;
use std::cell::RefCell;

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
    let new_element = Rc::new(RefCell::new(Element {
        prev: None,
        next: None,
        point: p,
    }));

    if let Some(existing_element) = e {
        let mut existing_element_borrowed = existing_element.borrow_mut();
        let next_element = existing_element_borrowed.next.clone();

        new_element.borrow_mut().next = next_element.clone();
        new_element.borrow_mut().prev = Some(existing_element.clone());

        if let Some(mut next_element_unwrapped) = next_element {
            next_element_unwrapped.borrow_mut().prev = Some(new_element.clone());
        }

        existing_element_borrowed.next = Some(new_element.clone());
    } else {
        new_element.borrow_mut().prev = Some(new_element.clone());
        new_element.borrow_mut().next = Some(new_element.clone());
    }

    new_element
}

type ByMaxX = Vec<Vec<Point>>;
