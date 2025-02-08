
use std::ops;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn are_all_outside(m: Point, k: Point, p_index: usize, outer: &[Point]) -> bool {
    let mut all_outside = true;
    for i in 0..outer.len() {
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
    let cross_product_1 = (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y);
    let cross_product_2 = (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y);
    let cross_product_3 = (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y);

    cross_product_1 >= 0.0 && cross_product_2 >= 0.0 && cross_product_3 >= 0.0
}
