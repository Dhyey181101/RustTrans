

use std::f64;
use std::fmt;
use std::error;
use std::ops;

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

fn find_k(m: Point, outer: Vec<Point>) -> (Point, usize, usize, MyError) {
    let mut k = Point { x: f64::INFINITY, y: m.y };
    let mut k1 = 0;
    let mut k2 = 0;
    let mut err: MyError = MyError::NoError;

    for i in (0..outer.len() - 1).rev() {
        let j = if i == 0 { outer.len() - 1 } else { i - 1 };

        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = m.sub(outer[i]);
        let v2 = outer[j].sub(outer[i]);

        let t1 = v2.cross(v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < k.x {
                k.x = t1 + m.x;
                k.y = m.y;
                k1 = i;
                k2 = j;
            }
        } else {
            err = MyError::ProblematicData;
            return (k, k1, k2, err);
        }
    }

    (k, k1, k2, err)
}

#[derive(Debug, Clone, Copy)]
enum MyError {
    NoError,
    ProblematicData,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::NoError => write!(f, "no error"),
            MyError::ProblematicData => write!(f, "cannot calculate intersection, problematic data"),
        }
    }
}

impl error::Error for MyError {}

impl ops::Not for MyError {
    type Output = MyError;

    fn not(self) -> MyError {
        match self {
            MyError::NoError => MyError::ProblematicData,
            MyError::ProblematicData => MyError::NoError,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x - other.x).abs() < 1e-9 && (self.y - other.y).abs() < 1e-9
    }
}

