
struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
}

fn remove(e: &mut Element) {
    if let Some(prev) = e.prev.as_deref_mut() {
        if let Some(next) = e.next.as_deref_mut() {
            prev.next = next.next.take();
            next.prev = prev.prev.take();
        }
    }
}
