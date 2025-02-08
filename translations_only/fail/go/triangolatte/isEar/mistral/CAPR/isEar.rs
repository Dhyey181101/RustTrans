

use std::mem;

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug)]
struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
    point: Point,
}

fn is_ear(p: &mut Element) -> bool {
    let a = &(*p.prev.as_mut().unwrap().clone()).point;
    let b = &p.point;
    let c = &(*p.next.as_mut().unwrap().clone()).point;
    if is_reflex(a, b, c) {
        return false;
    }

    let mut r = &mut *p.next.as_mut().unwrap().next.as_mut().unwrap().clone();
    loop {
        let inside = is_inside_triangle(a, b, c, &r.point);
        let reflex = is_reflex(
            &r.prev.as_mut().unwrap().clone().point,
            &r.point,
            &r.next.as_mut().unwrap().clone().point,
        );
        if inside && reflex {
            return false;
        }
        r = match r.next {
            Some(ref mut n) => n,
            None => {
                let tmp = mem::replace(&mut p.next, None);
                p.next = tmp.and_then(|n| n.prev);
                p.next.as_mut().unwrap();
                break;
            }
        };
    }
    true
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0 &&
    (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0 &&
    (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

