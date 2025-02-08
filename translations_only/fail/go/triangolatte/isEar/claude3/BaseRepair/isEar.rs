

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
    point: Point,
}

fn is_ear(p: &Element) -> bool {
    let a = p.prev.as_ref().map_or(&p.point, |prev| &prev.point);
    let b = &p.point;
    let c = p.next.as_ref().map_or(&p.point, |next| &next.point);

    if is_reflex(a, b, c) {
        return false;
    }

    let mut r = p.next.as_deref().and_then(|next| next.next.as_deref());
    while let Some(r_elem) = r {
        if r_elem.prev.as_deref() != Some(p) {
            let r_point = &r_elem.point;
            if is_inside_triangle(a, b, c, r_point)
                && is_reflex(
                    r_elem.prev.as_ref().and_then(|prev| prev.next.as_ref()).map_or(b, |next| &next.point),
                    r_point,
                    r_elem.next.as_ref().map_or(b, |next| &next.point),
                )
            {
                return false;
            }
        }
        r = r_elem.next.as_deref();
    }

    true
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0
        && (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0
        && (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

