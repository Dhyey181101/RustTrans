
use std::f64::consts::PI;
use std::f64::EPSILON;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn angle(&self, other: &Point) -> f64 {
        (self.y - other.y).atan2(self.x - other.x)
    }
}

#[derive(Debug, Clone, Copy)]
struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

fn area(a: &Point, b: &Point, c: &Point) -> f64 {
    let ab = a.distance(b);
    let bc = b.distance(c);
    let ca = c.distance(a);
    let s = (ab + bc + ca) / 2.0;
    (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
}

fn is_convex(a: &Point, b: &Point, c: &Point) -> bool {
    let ab = a.angle(b);
    let bc = b.angle(c);
    let ca = c.angle(a);
    ab < bc && bc < ca || ca < ab && ab < bc
}

fn contains(a: &Point, b: &Point, c: &Point, point: &Point) -> bool {
    let area1 = area(a, b, point);
    let area2 = area(b, c, point);
    let area3 = area(c, a, point);
    area1 + area2 + area3 < area(a, b, c) + EPSILON
}

fn polygon(points: &[Point]) -> Result<Vec<Triangle>, String> {
    if points.len() < 3 {
        return Err("Cannot triangulate less than three points".to_string());
    }

    let mut triangles = Vec::new();
    let mut points = points.to_vec();

    while points.len() >= 3 {
        let mut ear = None;

        for i in 0..points.len() {
            let a = points[(i - 1 + points.len()) % points.len()];
            let b = points[i];
            let c = points[(i + 1) % points.len()];

            if is_ear(&a, &b, &c) {
                ear = Some(i);
                break;
            }
        }

        if let Some(ear) = ear {
            let a = points[(ear - 1 + points.len()) % points.len()];
            let b = points[ear];
            let c = points[(ear + 1) % points.len()];

            triangles.push(Triangle { a, b, c });
            points.remove(ear);
        } else {
            return Err("Cannot triangulate polygon".to_string());
        }
    }

    Ok(triangles)
}

fn is_ear(a: &Point, b: &Point, c: &Point) -> bool {
    let ab = a.angle(b);
    let bc = b.angle(c);
    let ca = c.angle(a);

    ab < bc && bc < ca || ca < ab && ab < bc && is_convex(a, b, c)
}

fn main() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(0.0, 1.0),
    ];

    match polygon(&points) {
        Ok(triangles) => {
            for triangle in triangles {
                println!("{:?}", triangle);
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
