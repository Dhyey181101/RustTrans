
use std::f64::consts::PI;

#[derive(Clone, Copy)]
enum Joint {
    Normal = 0,
    Miter = 1,
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
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

    fn sub(&self, r: Point) -> Point {
        Point {
            x: self.x - r.x,
            y: self.y - r.y,
        }
    }

    fn dot(&self, r: Point) -> f64 {
        self.x * r.x + self.y * r.y
    }

    fn normalize(&self) -> Point {
        self.scale(1.0)
    }
}

fn line(joint: Joint, points: Vec<Point>, width: f64) -> Result<Vec<f64>, &'static str> {
    match joint {
        Joint::Normal => Ok(normal(&points, width)),
        Joint::Miter => Ok(miter(&points, width)),
        _ => Err("Unrecognized joint type"),
    }
}

fn normal(points: &Vec<Point>, width: f64) -> Vec<f64> {
    let mut width = width / 2.0;
    let mut triangles = Vec::with_capacity(points.len() * 12);
    for i in 0..=points.len() - 2 {
        let dx = points[i + 1].x - points[i].x;
        let dy = points[i + 1].y - points[i].y;
        let n1 = Point { x: dy, y: -dx }.scale(width);
        let n2 = Point { x: -dy, y: dx }.scale(width);

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

fn miter(points: &Vec<Point>, width: f64) -> Vec<f64> {
    let width = width / 2.0;
    let mut triangles = Vec::with_capacity(points.len() * 12);
    let mut dx = points[1].x - points[0].x;
    let mut dy = points[1].y - points[0].y;

    let mut n2 = calculate_normals(dx, dy);
    let mut miter2 = n2[0].scale(width);

    for i in 1..points.len() - 1 {
        let n1 = n2;
        let miter1 = miter2;

        dx = points[i + 1].x - points[i].x;
        dy = points[i + 1].y - points[i].y;

        n2 = calculate_normals(dx, dy);

        let tangent = (points[i + 1].sub(points[i])).normalize().add((points[i].sub(points[i - 1])).normalize()).normalize();

        let unit_miter = Point { x: -tangent.y, y: tangent.x };

        let miter_length = width / unit_miter.dot(n1[0]);
        miter2 = unit_miter.scale(miter_length);

        let v0 = points[i].sub(miter2).x;
        let v1 = points[i].sub(miter2).y;
        let v2 = points[i - 1].sub(miter1).x;
        let v3 = points[i - 1].sub(miter1).y;
        let v4 = points[i - 1].add(miter1).x;
        let v5 = points[i - 1].add(miter1).y;
        let v6 = points[i - 1].add(miter1).x;
        let v7 = points[i - 1].add(miter1).y;
        let v8 = points[i].add(miter2).x;
        let v9 = points[i].add(miter2).y;
        let v10 = points[i].sub(miter2).x;
        let v11 = points[i].sub(miter2).y;

        triangles.extend_from_slice(&[v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11]);
    }

    triangles
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        Point { x: y, y: -x }.normalize(),
        Point { x: -y, y: x }.normalize(),
    ]
}
