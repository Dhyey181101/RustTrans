
use std::boxed::Box;

type Point = (f64, f64);

fn areAllOutside(m: Point, k: Point, p_index: usize, outer: Vec<Point>) -> bool {
    let mut all_outside = true;
    for (i, _) in outer.iter().enumerate() {
        if i == p_index {
            continue;
        }

        if is_inside_triangle(m, k, outer[p_index], outer[i]) {
            all_outside = false;
        }
    }
    all_outside
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    (c.0 - p.0) * (a.1 - p.1) - (a.0 - p.0) * (c.1 - p.1) >= 0.0
        && (a.0 - p.0) * (b.1 - p.1) - (b.0 - p.0) * (a.1 - p.1) >= 0.0
        && (b.0 - p.0) * (c.1 - p.1) - (c.0 - p.0) * (b.1 - p.1) >= 0.0
}
