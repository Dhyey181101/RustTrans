

use std::f64;
use std::fmt;
use std::error;
use std::ops;
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

fn find_k(m: Point, outer: Vec<Point>) -> boxed::Box<Result<(Point, usize, usize), MyError>> {
    for i in (0..outer.len()).rev() {
        let j = (i + 1) % outer.len();
        let v1 = m.sub(outer[i]);
        let v2 = outer[j].sub(outer[i]);

        let t1 = v2.cross(v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < 0.0 {
                let k = Point {
                    x: t1 + m.x,
                    y: m.y,
                };
                return boxed::Box::new(Ok((k, i, j)));
            }
        }
    }

    boxed::Box::new(Err(MyError(String::from("cannot calculate intersection, problematic data"))))
}

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for MyError {}

impl From<String> for MyError {
    fn from(s: String) -> Self {
        MyError(s)
    }
}

impl From<&'static str> for MyError {
    fn from(s: &'static str) -> Self {
        MyError(s.to_string())
    }
}

