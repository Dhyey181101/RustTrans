
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
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
    let (c_minus_p, a_minus_p, c_minus_p_cross_a_minus_p) =
        (c - p, a - p, (c - p).cross(&(a - p)));
    let (a_minus_p_cross_b_minus_p, b_minus_p, b_minus_p_cross_c_minus_p) =
        ((a - p).cross(&(b - p)), b - p, (b - p).cross(&(c - p)));

    c_minus_p_cross_a_minus_p >= 0.0
        && a_minus_p_cross_b_minus_p >= 0.0
        && b_minus_p_cross_c_minus_p >= 0.0
}

impl Point {
    fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
}
