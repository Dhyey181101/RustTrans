

use std::boxed::Box;

#[derive(PartialEq)]
struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
    point: Point,
}

#[derive(PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn point(e: &Element) -> &Point {
    &e.point
}

fn is_ear(p: &Element) -> bool {
    let a = point(&*p.prev.as_ref().unwrap());
    let b = point(p);
    let c = point(&**p.next.as_ref().unwrap().next.as_ref().unwrap());

    if is_reflex(a, b, c) {
        return false;
    }

    let r = p.next.as_ref().unwrap().next.as_ref().unwrap();
    let mut cur = &**r.next.as_ref().unwrap();
    while cur.prev.as_ref().unwrap() != &*p.prev.as_ref().unwrap() {
        let inside = is_inside_triangle(a, b, c, point(cur));
        let reflex = is_reflex(point(&*cur.prev.as_ref().unwrap()), point(cur), point(&*cur.next.as_ref().unwrap()));

        if inside && reflex {
            return false;
        }

        cur = cur.next.as_ref().unwrap();
    }

    true
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    let left_a = (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y);
    let left_b = (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y);
    let left_c = (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y);

    left_a >= 0.0 && left_b >= 0.0 && left_c >= 0.0
}

