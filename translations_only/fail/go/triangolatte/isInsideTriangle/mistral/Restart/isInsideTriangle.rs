

use std::cmp::Ordering;

pub struct Point {
    x: f64,
    y: f64,
}

pub fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    let check_side = |point1: &Point, point2: &Point, point3: &Point| {
        (point3.x - point1.x) * (point2.y - point1.y)
            - (point2.x - point1.x) * (point3.y - point1.y)
    };
    let left_side = check_side(&c, &a, &p);
    let right_side = check_side(&a, &b, &p);
    let bottom_side = check_side(&b, &c, &p);
    left_side >= 0.0 && right_side >= 0.0 && bottom_side >= 0.0
}

