
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
    let p_norm = normalize(p);
    assert!((p_norm.x * p_norm.x + p_norm.y * p_norm.y - 1.0).abs() < 0.0001);
}

#[test]
fn test_scale() {
    let p = Point { x: 3.0, y: 4.0 };
    let p_scaled = scale(p, 2.0);
    assert_eq!(p_scaled.x, 6.0 / 5.0);
    assert_eq!(p_scaled.y, 8.0 / 5.0);
}
