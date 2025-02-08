
use std::collections::LinkedList;

struct Point {
    x: f64,
    y: f64,
}

fn find_closest(m: &Point, k: &Point, p_index: usize, outer: &Vec<Point>) -> usize {
    let mut reflex = LinkedList::new();
    let n = outer.len();
    for i in 0..n {
        let not_inside = !is_inside_triangle(m, k, &outer[p_index], &outer[i]);
        let prev = cyclic(i as isize - 1, n);
        let next = cyclic(i as isize + 1, n);
        let not_reflex = !is_reflex(&outer[prev], &outer[i], &outer[next]);
        if not_inside || not_reflex {
            continue;
        }
        reflex.push_back(i);
    }
    let mut closest = 0;
    let mut max_dist = 0.0;

    for r in reflex.iter() {
        let i = *r;
        let dist = distance2(&outer[i], &outer[closest]);
        if dist > max_dist {
            closest = i;
            max_dist = dist;
        }
    }
    closest
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0 &&
    (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0 &&
    (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

fn cyclic(i: isize, n: usize) -> usize {
    ((i % n as isize + n as isize) % n as isize) as usize
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn distance2(p: &Point, r: &Point) -> f64 {
    (p.x - r.x) * (p.x - r.x) + (p.y - r.y) * (p.y - r.y)
}
