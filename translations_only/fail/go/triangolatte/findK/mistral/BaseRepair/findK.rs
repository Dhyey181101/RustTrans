

use std::f64;
use std::fmt;
use std::boxed;

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

#[derive(Debug)]
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot calculate intersection, problematic data")
    }
}

impl std::error::Error for MyError {}

fn find_k(m: Point, outer: Vec<Point>) -> boxed::Box<(Point, usize, usize, Result<(), MyError>)> {
    for i in (0..outer.len()).rev() {
        let j = (i + 1) % outer.len();
        let v1 = m.sub(outer[i]);
        let v2 = outer[j].sub(outer[i]);

        let t1 = v2.cross(v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < 0.0 {
                return boxed::Box::new((Point {
                    x: t1 + m.x,
                    y: m.y,
                }, i, j, Ok(())));
            }
        }
    }

    boxed::Box::new((Point { x: 0.0, y: 0.0 }, 0, 0, Err(MyError)))
}

