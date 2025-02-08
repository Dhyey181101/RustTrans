

use std::cmp::Ordering;

pub struct Point {
    x: f64,
    y: f64,
}

pub fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    let is_left_of_ab = (b.y - a.y) * (p.x - a.x) + (-b.x + a.x) * (p.y - a.y);
    let is_left_of_bc = (c.y - b.y) * (p.x - b.x) + (-c.x + b.x) * (p.y - b.y);
    let is_left_of_ca = (a.y - c.y) * (p.x - c.x) + (-a.x + c.x) * (p.y - c.y);

    (is_left_of_ab < 0.0) && (is_left_of_bc < 0.0) && (is_left_of_ca < 0.0)
}

