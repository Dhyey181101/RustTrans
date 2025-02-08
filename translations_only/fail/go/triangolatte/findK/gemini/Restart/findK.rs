
use std::f64::consts::PI;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn cross(&self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
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

fn find_k(m: Point, outer: &[Point]) -> Result<(Point, usize, usize), &'static str> {
    for (i, j) in (0..outer.len()).rev().zip(0..outer.len()) {
        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = m.sub(outer[i]);
        let v2 = outer[j].sub(outer[i]);

        let t1 = v2.cross(v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < 0.0 {
                return Ok((Point { x: t1 + m.x, y: m.y }, i, j));
            }
        } else {
            return Err("cannot calculate intersection, problematic data");
        }
    }

    Err("no intersection found")
}

fn main() {
    let m = Point { x: 0.0, y: 0.0 };
    let outer = vec![
        Point { x: -1.0, y: -1.0 },
        Point { x: 1.0, y: -1.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: -1.0, y: 1.0 },
    ];

    match find_k(m, &outer) {
        Ok((k, k1, k2)) => println!("k: {:?}, k1: {}, k2: {}", k, k1, k2),
        Err(e) => println!("Error: {}", e),
    }
}
