
use std::f64;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn normalize(self) -> Point {
        self.scale(1.0)
    }

    fn scale(self, f: f64) -> Point {
        let norm = f64::sqrt(self.x * self.x + self.y * self.y);
        Point {
            x: self.x / norm * f,
            y: self.y / norm * f,
        }
    }

    fn sub(self, r: Point) -> Point {
        Point {
            x: self.x - r.x,
            y: self.y - r.y,
        }
    }

    fn add(self, r: Point) -> Point {
        Point {
            x: self.x + r.x,
            y: self.y + r.y,
        }
    }

    fn dot(self, r: Point) -> f64 {
        self.x * r.x + self.y * r.y
    }
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        Point { x: y, y: -x }.normalize(),
        Point { x: -y, y: x }.normalize(),
    ]
}

fn miter(points: Vec<Point>, width: f64) -> Vec<f64> {
    if points.len() < 2 {
        panic!("Input is invalid, crash gracefully");
    }

    let width = width / 2.0;
    let mut triangles = Vec::with_capacity(points.len() * 12);
    let mut dx;
    let mut dy;
    let mut miter1 = Point { x: 0.0, y: 0.0 };
    let mut miter2 = Point { x: 0.0, y: 0.0 };
    let mut n1;
    let mut n2 = [Point { x: 0.0, y: 0.0 }; 2];

    dx = points[1].x - points[0].x;
    dy = points[1].y - points[0].y;

    n2 = calculate_normals(dx, dy);
    miter2 = n2[0].scale(width);

    for i in 1..points.len() - 1 {
        n1 = n2;
        miter1 = miter2;

        dx = points[i + 1].x - points[i].x;
        dy = points[i + 1].y - points[i].y;

        n2 = calculate_normals(dx, dy);

        let tangent = (points[i + 1].sub(points[i])).normalize().add((points[i].sub(points[i - 1])).normalize()).normalize();
        let unit_miter = Point { x: -tangent.y, y: tangent.x };

        let miter_length = width / unit_miter.dot(n1[0]);
        miter2 = unit_miter.scale(miter_length);

        let (v0, v1) = (points[i].sub(miter2).x, points[i].sub(miter2).y);
        let (v2, v3) = (points[i - 1].sub(miter1).x, points[i - 1].sub(miter1).y);
        let (v4, v5) = (points[i - 1].add(miter1).x, points[i - 1].add(miter1).y);
        let (v6, v7) = (points[i - 1].add(miter1).x, points[i - 1].add(miter1).y);
        let (v8, v9) = (points[i].add(miter2).x, points[i].add(miter2).y);
        let (v10, v11) = (points[i].sub(miter2).x, points[i].sub(miter2).y);

        triangles.extend_from_slice(&[v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11]);
    }

    let n = points.len();
    let last_miter = n2[0].scale(width);

    let (v0, v1) = (points[n - 1].sub(last_miter).x, points[n - 1].sub(last_miter).y);
    let (v2, v3) = (points[n - 2].sub(miter1).x, points[n - 2].sub(miter1).y);
    let (v4, v5) = (points[n - 2].add(miter1).x, points[n - 2].add(miter1).y);
    let (v6, v7) = (points[n - 2].add(miter1).x, points[n - 2].add(miter1).y);
    let (v8, v9) = (points[n - 1].add(last_miter).x, points[n - 1].add(last_miter).y);
    let (v10, v11) = (points[n - 1].sub(last_miter).x, points[n - 1].sub(last_miter).y);

    triangles.extend_from_slice(&[v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11]);

    triangles
}
