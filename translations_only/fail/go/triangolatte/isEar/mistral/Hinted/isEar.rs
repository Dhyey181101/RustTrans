

use std::mem;
use std::option::Option::None;
use std::option::Option::Some;

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

fn is_ear(p: &Element) -> bool {
    let a = &p.prev.as_ref().unwrap().point;
    let b = &p.point;
    let c = &p.next.as_ref().unwrap().point;
    if is_reflex(a, b, c) {
        return false;
    }

    let mut r = &*p.next.as_ref().unwrap().next.as_ref().unwrap();
    while r != &*p.prev.as_ref().unwrap() {
        let inside = is_inside_triangle(a, b, c, &r.point);
        let reflex = is_reflex(&r.prev.as_ref().unwrap().point, &r.point, &r.next.as_ref().unwrap().point);
        if inside && reflex {
            return false;
        }
        r = &*r.next.as_ref().unwrap();
    }
    true
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    let check_side = |a: &Point, b: &Point, p: &Point| (b.x - p.x) * (a.y - p.y) - (a.x - p.x) * (b.y - p.y);
    check_side(c, a, p) >= 0.0 && check_side(a, b, p) >= 0.0 && check_side(b, c, p) >= 0.0
}

fn main() {
    // Test cases go here
}

