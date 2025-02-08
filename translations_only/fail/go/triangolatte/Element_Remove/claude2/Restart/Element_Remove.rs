
use std::mem;

#[derive(Clone)]
struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
}

fn remove(e: &mut Element) {
    if let Some(mut next) = mem::take(&mut e.next) {
        if let Some(mut prev) = mem::take(&mut e.prev) {
            set_next(&mut prev, Some(next.clone()));
            set_prev(&mut next, Some(prev));
        }
    }
}

fn set_prev(e: &mut Box<Element>, prev: Option<Box<Element>>) {
    e.prev = prev;
}

fn set_next(e: &mut Box<Element>, next: Option<Box<Element>>) {
    e.next = next;
}

