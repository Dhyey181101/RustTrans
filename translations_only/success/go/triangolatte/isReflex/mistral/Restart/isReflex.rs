

use std::ops::Mul;

pub struct Point {
    x: f64,
    y: f64,
}

pub fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    let vec_ab = vec2(b.x - a.x, b.y - a.y);
    let vec_bc = vec2(c.x - b.x, c.y - b.y);
    cross(vec_ab, vec_bc) < 0.0
}

pub fn vec2(x: f64, y: f64) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

pub fn cross(a: Vec2, b: Vec2) -> f64 {
    a.x * b.y - a.y * b.x
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

