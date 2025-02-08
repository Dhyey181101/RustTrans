
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    let c_minus_p = c - p;
    let a_minus_p = a - p;
    let c_minus_p_cross_a_minus_p = c_minus_p.cross(&a_minus_p);
    let a_minus_p = a - p;
    let b_minus_p = b - p;
    let a_minus_p_cross_b_minus_p = a_minus_p.cross(&b_minus_p);
    let b_minus_p = b - p;
    let c_minus_p = c - p;
    let b_minus_p_cross_c_minus_p = b_minus_p.cross(&c_minus_p);

    c_minus_p_cross_a_minus_p >= 0.0
        && a_minus_p_cross_b_minus_p >= 0.0
        && b_minus_p_cross_c_minus_p >= 0.0
}

impl Point {
    fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}
