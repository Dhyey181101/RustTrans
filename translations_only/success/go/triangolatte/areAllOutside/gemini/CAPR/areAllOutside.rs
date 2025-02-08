
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
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
    let (c_x_p_x, a_y_p_y, a_x_p_x, c_y_p_y, b_x_p_x, b_y_p_y) =
        (c.x - p.x, a.y - p.y, a.x - p.x, c.y - p.y, b.x - p.x, b.y - p.y);
    (c_x_p_x * a_y_p_y - a_x_p_x * c_y_p_y) >= 0.0
        && (a_x_p_x * b_y_p_y - b_x_p_x * a_y_p_y) >= 0.0
        && (b_x_p_x * c_y_p_y - c_x_p_x * b_y_p_y) >= 0.0
}
