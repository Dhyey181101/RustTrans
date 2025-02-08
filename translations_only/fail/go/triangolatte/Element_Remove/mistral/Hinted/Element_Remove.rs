
use std::mem;

struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
}

impl Element {
    fn new() -> Element {
        Element { prev: None, next: None }
    }

    fn remove(&mut self) {
        if let Some(ref mut next) = self.next {
            next.prev = self.prev.take();
        }
        if let Some(ref mut prev) = self.prev {
            prev.next = self.next.take();
        }
        mem::forget(self);
    }
}

fn main() {
    let mut e = Element::new();
    e.next = Some(Box::new(Element::new()));
    e.next.as_mut().unwrap().prev = Some(Box::new(Element::new()));
    e.remove();
}
