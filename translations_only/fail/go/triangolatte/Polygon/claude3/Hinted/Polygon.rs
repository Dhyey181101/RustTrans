
#[derive(Clone)]
struct Element {
    prev: Box<Element>,
    next: Box<Element>,
    point: (i32, i32),
}

fn remove(element: &mut Box<Element>) {
    let mut prev = element.prev.clone();
    let mut next = element.next.clone();
    prev.next = next.clone();
    next.prev = prev.clone();
}
