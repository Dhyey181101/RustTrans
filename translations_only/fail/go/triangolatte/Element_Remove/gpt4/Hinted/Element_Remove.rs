
struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
}

fn remove(e: &mut Element) {
    match (&mut e.prev, &mut e.next) {
        (Some(prev), Some(next)) => {
            prev.next = Some(Box::new(Element {
                prev: prev.prev.take(),
                next: next.next.take(),
            }));
            next.prev = Some(Box::new(Element {
                prev: prev.prev.take(),
                next: next.next.take(),
            }));
        }
        _ => panic!("Input is invalid, crash gracefully"),
    }
}
