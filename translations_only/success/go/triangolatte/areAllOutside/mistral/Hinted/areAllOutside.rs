

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn areAllOutside(m: Point, k: Point, p_index: usize, outer: Vec<Point>) -> bool {
    let mut all_outside = true;
    for (i, _) in outer.iter().enumerate() {
        if i == p_index {
            continue;
        }

        if is_in_triangle(m, k, outer[p_index], outer[i]) {
            all_outside = false;
        }
    }
    all_outside
}

fn is_in_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    let ab_cross_cp = (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y);
    let bc_cross_ap = (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y);
    let ca_cross_bp = (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y);

    ab_cross_cp >= 0.0 && bc_cross_ap >= 0.0 && ca_cross_bp >= 0.0
}

fn main() {
    let m = Point { x: 5.562685375517167e-309, y: 0.0 };
    let k = Point { x: 0.0, y: 1.39064994160909e-309 };
    let p_index = 0;
    let outer: Vec<Point> = Vec::new();

    if m.x.is_nan() || m.y.is_nan() || k.x.is_nan() || k.y.is_nan() || p_index >= outer.len() {
        panic!("Input is invalid, crash gracefully");
    }
}

