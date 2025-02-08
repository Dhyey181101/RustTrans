
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn cross_product(p1: Point, p2: Point) -> f64 {
    p1.x * p2.y - p1.y * p2.x
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
    let p1 = cross_product(c - p, a - p);
    let p2 = cross_product(a - p, b - p);
    let p3 = cross_product(b - p, c - p);
    p1 >= 0.0 && p2 >= 0.0 && p3 >= 0.0
}
