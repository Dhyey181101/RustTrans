
struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
}

impl Element {
    fn remove(&mut self) {
        if let Some(ref mut prev) = self.prev {
            prev.next = self.next.take();
        }

        if let Some(ref mut next) = self.next {
            next.prev = self.prev.take();
        }
    }
}
