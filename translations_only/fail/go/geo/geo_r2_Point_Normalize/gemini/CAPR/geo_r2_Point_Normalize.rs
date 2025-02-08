
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn normalize(&self) -> Point {
        if self.x == 0.0 && self.y == 0.0 {
            *self
        } else {
            mul_point_f64(*self, 1.0 / self.norm())
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn add(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

pub fn sub(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}

pub fn div(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x / p2.x,
        y: p1.y / p2.y,
    }
}

pub fn mul(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x * p2.x,
        y: p1.y * p2.y,
    }
}

pub fn mul_point_f64(p: Point, m: f64) -> Point {
    Point {
        x: p.x * m,
        y: p.y * m,
    }
}

pub fn mul_f64_point(m: f64, p: Point) -> Point {
    Point {
        x: p.x * m,
        y: p.y * m,
    }
}
