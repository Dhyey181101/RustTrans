
use std::error::Error;

#[derive(Copy, Clone)]
enum Joint {
    Normal = 0,
    Miter = 1,
}

fn line(joint: Joint, points: &[Point], width: f64) -> Result<Vec<f64>, Box<dyn Error>> {
    match joint {
        Joint::Normal => Ok(normal(points, width)),
        Joint::Miter => Ok(miter(points, width)),
        _ => Err("Unrecognized joint type".into()),
    }
}

fn normal(points: &[Point], width: f64) -> Vec<f64> {
    let mut triangles = Vec::new();
    let width = width / 2.0;

    for i in 0..(points.len() - 1) {
        let dx = points[i + 1].x - points[i].x;
        let dy = points[i + 1].y - points[i].y;
        let n1 = Point { x: dy, y: -dx }.scale(width);
        let n2 = Point { x: -dy, y: dx }.scale(width);

        let v0 = points[i + 1].x + n2.x;
        let v1 = points[i + 1].y + n2.y;
        let v2 = points[i].x + n2.x;
        let v3 = points[i].y + n2.y;
        let v4 = points[i].x + n1.x;
        let v5 = points[i].y + n1.y;
        let v6 = points[i].x + n1.x;
        let v7 = points[i].y + n1.y;
        let v8 = points[i + 1].x + n1.x;
        let v9 = points[i + 1].y + n1.y;
        let v10 = points[i + 1].x + n2.x;
        let v11 = points[i + 1].y + n2.y;

        triangles.extend_from_slice(&[
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11,
        ]);
    }

    triangles
}

fn scale(p: Point, f: f64) -> Point {
    let norm = (p.x * p.x + p.y * p.y).sqrt();
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}

fn miter(points: &[Point], width: f64) -> Vec<f64> {
    let mut triangles = Vec::new();
    let width = width / 2.0;
    let mut dx;
    let mut dy;
    let mut miter1 = Point { x: 0.0, y: 0.0 };
    let mut miter2;
    let mut n1;
    let mut n2;

    dx = points[1].x - points[0].x;
    dy = points[1].y - points[0].y;

    n2 = calculate_normals(dx, dy);
    miter2 = scale(n2[0], width);

    for i in 1..(points.len() - 1) {
        n1 = n2;
        miter1 = miter2;

        dx = points[i + 1].x - points[i].x;
        dy = points[i + 1].y - points[i].y;

        n2 = calculate_normals(dx, dy);

        let tangent = normalize(sub(
            sub(points[i + 1], points[i]),
            sub(points[i], points[i - 1]),
        ));

        let unit_miter = Point {
            x: -tangent.y,
            y: tangent.x,
        };

        let miter_length = width / dot(unit_miter, n1[0]);
        miter2 = scale(unit_miter, miter_length);

        let v0 = points[i].x - miter2.x;
        let v1 = points[i].y - miter2.y;
        let v2 = points[i - 1].x - miter1.x;
        let v3 = points[i - 1].y - miter1.y;
        let v4 = points[i - 1].x + miter1.x;
        let v5 = points[i - 1].y + miter1.y;
        let v6 = points[i - 1].x + miter1.x;
        let v7 = points[i - 1].y + miter1.y;
        let v8 = points[i].x + miter2.x;
        let v9 = points[i].y + miter2.y;
        let v10 = points[i].x - miter2.x;
        let v11 = points[i].y - miter2.y;

        triangles.extend_from_slice(&[
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11,
        ]);
    }

    let n = points.len();
    let last_miter = scale(n2[0], width);

    let v0 = points[n - 1].x - last_miter.x;
    let v1 = points[n - 1].y - last_miter.y;
    let v2 = points[n - 2].x - miter1.x;
    let v3 = points[n - 2].y - miter1.y;
    let v4 = points[n - 2].x + miter1.x;
    let v5 = points[n - 2].y + miter1.y;
    let v6 = points[n - 2].x + miter1.x;
    let v7 = points[n - 2].y + miter1.y;
    let v8 = points[n - 1].x + last_miter.x;
    let v9 = points[n - 1].y + last_miter.y;
    let v10 = points[n - 1].x - last_miter.x;
    let v11 = points[n - 1].y - last_miter.y;

    triangles.extend_from_slice(&[
        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11,
    ]);

    triangles
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        Point { x: y, y: -x }.normalize(),
        Point { x: -y, y: x }.normalize(),
    ]
}

fn normalize(p: Point) -> Point {
    scale(p, 1.0)
}

fn sub(p: Point, r: Point) -> Point {
    Point {
        x: p.x - r.x,
        y: p.y - r.y,
    }
}

fn dot(p: Point, r: Point) -> f64 {
    p.x * r.x + p.y * r.y
}

#[derive(Copy, Clone)]
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

    fn normalize(&self) -> Point {
        self.scale(1.0)
    }
}
