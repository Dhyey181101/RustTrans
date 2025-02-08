
use std::cell::RefCell;
use std::collections::LinkedList;

type Point = (f64, f64);

fn find_closest(m: Point, k: Point, p_index: usize, outer: Vec<Point>) -> usize {
    let reflex = RefCell::new(LinkedList::new());
    let n = outer.len();
    for i in 0..n {
        let not_inside = !is_inside_triangle(m, k, outer[p_index], outer[i]);
        let (prev, next) = (cyclic(i - 1, n), cyclic(i + 1, n));
        let not_reflex = !is_reflex(outer[prev], outer[i], outer[next]);
        if not_inside || not_reflex {
            continue;
        }
        reflex.borrow_mut().push_back(i);
    }
    let mut closest = 0;
    let mut max_dist = 0.0;

    for r in reflex.borrow().iter() {
        let i = *r;
        let dist = outer[i].1.powf(2.0) + outer[i].0.powf(2.0) - outer[closest].1.powf(2.0) - outer[closest].0.powf(2.0);
        if dist > max_dist {
            closest = i;
            max_dist = dist;
        }
    }
    closest
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    (c.0 - p.0) * (a.1 - p.1) - (a.0 - p.0) * (c.1 - p.1) >= 0.0 &&
    (a.0 - p.0) * (b.1 - p.1) - (b.0 - p.0) * (a.1 - p.1) >= 0.0 &&
    (b.0 - p.0) * (c.1 - p.1) - (c.0 - p.0) * (b.1 - p.1) >= 0.0
}

fn cyclic(i: usize, n: usize) -> usize {
    (i % n + n) % n
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.0 - a.0) * (c.1 - b.1) - (c.0 - b.0) * (b.1 - a.1) < 0.0
}

fn distance2(p: Point, r: Point) -> f64 {
    (p.0 - r.0).powf(2.0) + (p.1 - r.1).powf(2.0)
}
