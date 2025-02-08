
use std::ops::Add;

#[derive(PartialEq, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

struct Element {
    prev: Box<Element>,
    next: Box<Element>,
    point: Point,
}

fn is_ear(p: &Element) -> bool {
    let a = p.prev.point;
    let b = p.point;
    let c = p.next.point;
    
    if is_reflex(a, b, c) {
        return false;
    }

    let mut r = &p.next.next;
    while !eq_box_element(r, &p.prev) {
        let inside = is_inside_triangle(a, b, c, r.point);
        let reflex = is_reflex(r.prev.point, r.point, copy_point(r.next.point));
        if inside && reflex {
            return false;
        }
        r = &r.next;
    }
    
    true
}

fn eq_box_element(a: &Box<Element>, b: &Box<Element>) -> bool {
    std::ptr::eq(a, b)
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0
        && (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0
        && (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

fn copy_point(p: Point) -> Point {
    p
}

