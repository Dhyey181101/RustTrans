
use std::mem;

struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
}

impl Element {
    fn new() -> Element {
        Element { prev: None, next: None }
    }
}

impl Element {
    fn remove(&mut self) {
        match self.prev.take() {
            Some(ref mut prev) => prev.next = self.next.take(),
            None => (),
        }
        match self.next.take() {
            Some(ref mut next) => next.prev = self.prev.take(),
            None => (),
        }
    }
}
