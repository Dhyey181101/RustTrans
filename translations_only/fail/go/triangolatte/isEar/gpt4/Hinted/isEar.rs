
use std::rc::Rc;
use std::cell::RefCell;

struct Element {
    prev: Option<Rc<RefCell<Element>>>,
    next: Option<Rc<RefCell<Element>>>,
    point: Point,
}

struct Point {
    x: f64,
    y: f64,
}

fn is_ear(p: &Rc<RefCell<Element>>) -> bool {
    let p_borrow = p.borrow();
    if let (Some(prev), Some(next)) = (&p_borrow.prev, &p_borrow.next) {
        let a = &prev.borrow().point;
        let b = &p_borrow.point;
        let c = &next.borrow().point;
        if is_reflex(a, b, c) {
            return false;
        }

        let mut r = next.borrow().next.clone();
        while let Some(r_strong) = r {
            if Rc::ptr_eq(&r_strong, &prev) {
                break;
            }
            let r_point = &r_strong.borrow().point;
            let inside = is_inside_triangle(a, b, c, r_point);
            let reflex = is_reflex(
                &r_strong.borrow().prev.as_ref().unwrap().borrow().point,
                r_point,
                &r_strong.borrow().next.as_ref().unwrap().borrow().point,
            );
            if inside && reflex {
                return false;
            }
            r = r_strong.borrow().next.clone();
        }
        true
    } else {
        false
    }
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0 &&
    (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0 &&
    (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}
