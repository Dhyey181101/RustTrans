
struct Element {
    prev: Box<Option<Element>>,
    next: Box<Option<Element>>,
    point: Point,
}

struct Point {
    x: f64,
    y: f64,
}

fn is_ear(p: &Element) -> bool {
    let a = &p.prev.as_ref().as_ref().unwrap().point;
    let b = &p.point;
    let c = &p.next.as_ref().as_ref().unwrap().point;
    if is_reflex(a, b, c) {
        return false;
    }

    let mut r = &p.next.as_ref().as_ref().unwrap().next;
    while !std::ptr::eq(
        r.as_ref().as_ref().unwrap() as *const Element,
        p.prev.as_ref().as_ref().unwrap() as *const Element,
    ) {
        let r_point = &r.as_ref().as_ref().unwrap().point;
        let inside = is_inside_triangle(a, b, c, r_point);
        let reflex = is_reflex(
            &r.as_ref().as_ref().unwrap().prev.as_ref().as_ref().unwrap().point,
            r_point,
            &r.as_ref().as_ref().unwrap().next.as_ref().as_ref().unwrap().point,
        );
        if inside && reflex {
            return false;
        }
        r = &r.as_ref().as_ref().unwrap().next;
    }
    true
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    ((c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0) &&
    ((a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0) &&
    ((b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0)
}
