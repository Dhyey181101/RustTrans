
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Element {
    prev: Option<Box<Element>>,
    next: Option<Box<Element>>,
    point: Point,
}

fn is_ear(p: &Element) -> bool {
    let a = p.prev.as_ref().map_or(Point { x: f64::NAN, y: f64::NAN }, |p| p.point);
    let b = p.point;
    let c = p.next.as_ref().map_or(Point { x: f64::NAN, y: f64::NAN }, |p| p.point);

    if is_reflex(a, b, c) {
        return false;
    }

    let mut r = p.next.as_ref().and_then(|p| p.next.as_ref());
    while let Some(r_elem) = r {
        if let Some(prev_elem) = r_elem.prev.as_ref() {
            if prev_elem.point == a {
                break;
            }
            let r_point = r_elem.point;
            let inside = is_inside_triangle(a, b, c, r_point);
            let reflex = is_reflex(
                prev_elem.point,
                r_point,
                r_elem.next.as_ref().map_or(Point { x: f64::NAN, y: f64::NAN }, |p| p.point),
            );
            if inside && reflex {
                return false;
            }
        }
        r = r_elem.next.as_ref();
    }
    true
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0
        && (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0
        && (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}
