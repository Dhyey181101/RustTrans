
use std::f64::consts::PI;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn normalize(&self) -> Point {
        self.scale(1.0)
    }

    fn scale(&self, f: f64) -> Point {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        Point {
            x: self.x / norm * f,
            y: self.y / norm * f,
        }
    }
}

fn main() {
    let p = Point { x: 5.3779854396973e-310, y: 7.547003473039939e+168 };
    println!("{:?}", p.normalize()); // {'x': 0.0, 'y': 0.0}

    let p = Point { x: -1.9999406742134578e+304, y: 1.9872184857472316e-168 };
    println!("{:?}", p.normalize()); // {'x': -0.0, 'y': 0.0}

    let p = Point { x: 1.9287207540608773e-168, y: 7.290256734134497e-304 };
    println!("{:?}", p.normalize()); // Input is invalid, crash gracefully

    let p = Point { x: 2.0044188810539497e-289, y: 1.3646e-320 };
    println!("{:?}", p.normalize()); // Input is invalid, crash gracefully
}
