
use std::boxed::Box;

#[derive(Clone)]
struct Element {
    prev: Box<Element>,
    next: Box<Element>,
}

fn remove(e: &mut Element) {
    *e.prev.next = *e.next.clone();
    *e.next.prev = *e.prev.clone();
}

