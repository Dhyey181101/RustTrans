
use std::cell::RefCell;
use std::collections::LinkedList;

type Point = (f64, f64);

fn find_closest(m: Point, k: Point, p_index: usize, outer: Vec<Point>) -> usize {
    let reflex = RefCell::new(LinkedList::new());
    let n = outer.len();
    for i in 0..n {
        let not_inside = !is_inside_triangle(m, k, outer[p_index], outer[i]);
        let (prev, next) = (cyclic(i as i32 - 1, n as i32), cyclic(i as i32 + 1, n as i32));
        let not_reflex = !is_reflex(outer[prev as usize], outer[i as usize], outer[next as usize]);
        if not_inside || not_reflex {
            continue;
        }
        reflex.borrow_mut().push_back(i);
    }
    let mut closest = 0;
    let mut max_dist = 0.0;

    for r in reflex.borrow().iter() {
        let i = *r;
        let dist = dist_sqrd(outer[i], outer[closest]);
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

fn cyclic(i: i32, n: i32) -> i32 {
    (i % n + n) % n
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.0 - a.0) * (c.1 - b.1) - (c.0 - b.0) * (b.1 - a.1) < 0.0
}

fn dist_sqrd(a: Point, b: Point) -> f64 {
    (a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)
}
