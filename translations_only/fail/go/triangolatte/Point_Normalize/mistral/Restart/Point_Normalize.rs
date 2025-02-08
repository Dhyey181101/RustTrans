
use std::f64;
use std::ops::Mul;

fn normalize(p: Point) -> Point {
    let norm = (p.x * p.x + p.y * p.y).sqrt();
    Point {
        x: p.x / norm,
        y: p.y / norm,
    }
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x * p.x + p.y * p.y).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn test_normalize() {
    let p = Point { x: 3.0, y: 4.0 };
    assert_eq!(normalize(p), Point { x: 0.6, y: 0.8 });
}

#[test]
fn test_scale() {
    let p = Point { x: 3.0, y: 4.0 };
    assert_eq!(scale(p, 10.0), Point { x: 18.0, y: 24.0 });
}
