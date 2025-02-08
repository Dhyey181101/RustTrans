

use std::f64;
use std::fmt;
use std::boxed;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn sub(&self, r: Point) -> Point {
        Point {
            x: self.x - r.x,
            y: self.y - r.y,
        }
    }

    fn cross(&self, r: Point) -> f64 {
        self.x * r.y - self.y * r.x
    }
}

fn find_k(m: Point, outer: Vec<Point>) -> (Point, usize, usize, boxed::Box<dyn std::error::Error>) {
    let mut err: Option<boxed::Box<dyn std::error::Error>> = None;
    let mut k = Point { x: f64::INFINITY, y: m.y };
    let mut k1 = 0;
    let mut k2 = 0;

    for i in (0..outer.len()).rev() {
        let j = (i + 1) % outer.len();

        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = m.sub(outer[i]);
        let v2 = outer[j].sub(outer[i]);

        let t1 = v2.cross(v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < k.x {
                k = Point { x: t1 + m.x, y: m.y };
                k1 = i;
                k2 = j;
            }
        } else {
            if err.is_none() {
                err = Some(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "cannot calculate intersection, problematic data",
                )));
            }
        }
    }

    (k, k1, k2, err.unwrap())
}

fn main() {}

