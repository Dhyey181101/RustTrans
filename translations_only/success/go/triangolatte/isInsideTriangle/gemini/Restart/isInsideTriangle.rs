
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    let (c_minus_p, a_minus_p, b_minus_p) = (c - p, a - p, b - p);
    (c_minus_p.x * a_minus_p.y - a_minus_p.x * c_minus_p.y) >= 0.0
        && (a_minus_p.x * b_minus_p.y - b_minus_p.x * a_minus_p.y) >= 0.0
        && (b_minus_p.x * c_minus_p.y - c_minus_p.x * b_minus_p.y) >= 0.0
}
