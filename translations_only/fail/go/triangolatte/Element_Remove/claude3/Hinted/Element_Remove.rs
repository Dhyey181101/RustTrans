
#[derive(Clone, Default)]
pub struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
}

pub fn remove(e: &mut Element) {
    if let Some(prev) = e.prev.as_mut() {
        if let Some(next) = e.next.as_mut() {
            prev.next = next.next.take();
            if let Some(next_next) = next.next.as_mut() {
                next_next.prev = Some(Box::new(Element { prev: Some(prev.clone()), ..Default::default() }));
            }
        } else {
            prev.next = None;
        }
    } else if let Some(next) = e.next.as_mut() {
        next.prev = None;
    }
    e.prev = None;
    e.next = None;
}
