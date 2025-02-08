
use std::fmt;
use std::ops::Add;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
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

impl Point {
    fn scale(&self, f: f64) -> Point {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        Point {
            x: self.x / norm * f,
            y: self.y / norm * f,
        }
    }

    fn add(&self, r: Point) -> Point {
        Point {
            x: self.x + r.x,
            y: self.y + r.y,
        }
    }
}

fn normal(points: Vec<Point>, width: f64) -> Vec<f64> {
    let width = width / 2.0;
    let mut triangles = Vec::with_capacity(points.len() * 12);

    for i in 0..points.len() - 1 {
        let dx = points[i + 1].x - points[i].x;
        let dy = points[i + 1].y - points[i].y;
        let n1 = Point {
            x: dy * width,
            y: -dx * width,
        };
        let n2 = Point {
            x: -dy * width,
            y: dx * width,
        };

        let v0 = points[i + 1].add(n2).x;
        let v1 = points[i + 1].add(n2).y;
        let v2 = points[i].add(n2).x;
        let v3 = points[i].add(n2).y;
        let v4 = points[i].add(n1).x;
        let v5 = points[i].add(n1).y;
        let v6 = points[i].add(n1).x;
        let v7 = points[i].add(n1).y;
        let v8 = points[i + 1].add(n1).x;
        let v9 = points[i + 1].add(n1).y;
        let v10 = points[i + 1].add(n2).x;
        let v11 = points[i + 1].add(n2).y;

        triangles.extend_from_slice(&[v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11]);
    }

    triangles
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 0.0, y: 1.0 },
    ];

    let triangles = normal(points, 1.0);

    for i in 0..triangles.len() {
        if i % 2 == 0 {
            println!("[");
        }
        println!("\t{},", triangles[i]);
        if (i + 1) % 2 == 0 || i == triangles.len() - 1 {
            println!("\t]");
        }
    }
}
