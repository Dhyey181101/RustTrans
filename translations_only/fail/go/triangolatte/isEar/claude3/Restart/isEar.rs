
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
    let a = p.prev.as_ref().map_or(p.point, |prev| prev.point);
    let b = p.point;
    let c = p.next.as_ref().map_or(p.point, |next| next.point);

    if is_reflex(&a, &b, &c) {
        return false;
    }

    let mut r = p.next.as_deref();
    loop {
        if let Some(r_elem) = r {
            let next_r = r_elem.next.as_deref();
            if next_r == p.prev.as_deref() {
                break;
            }
            let inside = is_inside_triangle(&a, &b, &c, &r_elem.point);
            let reflex = is_reflex(&r_elem.prev.as_ref().map_or(r_elem.point, |prev| prev.point), &r_elem.point, &r_elem.next.as_ref().map_or(r_elem.point, |next| next.point));
            if inside && reflex {
                return false;
            }
            r = next_r;
        } else {
            break;
        }
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
