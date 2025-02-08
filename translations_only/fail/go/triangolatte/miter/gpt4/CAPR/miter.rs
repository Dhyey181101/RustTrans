
use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn miter(points: Vec<Point>, mut width: f64) -> Vec<f64> {
    width /= 2.0;
    let mut triangles = Vec::with_capacity(points.len() * 12);
    let mut dx;
    let mut dy;
    let mut miter1 = Point { x: 0.0, y: 0.0 };
    let mut miter2 = Point { x: 0.0, y: 0.0 };
    let mut n1;
    let mut n2 = [Point { x: 0.0, y: 0.0 }; 2];

    if points.len() > 1 {
        dx = points[1].x - points[0].x;
        dy = points[1].y - points[0].y;

        n2 = calculate_normals(dx, dy);
        miter2 = scale(n2[0], width);
    }

    for i in 1..points.len() - 1 {
        n1 = n2;
        miter1 = miter2;

        dx = points[i + 1].x - points[i].x;
        dy = points[i + 1].y - points[i].y;

        n2 = calculate_normals(dx, dy);

        let tangent = normalize(add(
            normalize(sub(points[i + 1], points[i])),
            normalize(sub(points[i], points[i - 1])),
        ));

        let unit_miter = Point {
            x: -tangent.y,
            y: tangent.x,
        };

        let miter_length = width / dot(unit_miter, n1[0]);
        miter2 = scale(unit_miter, miter_length);

        let (v0, v1) = (sub(points[i], miter2).x, sub(points[i], miter2).y);
        let (v2, v3) = (sub(points[i - 1], miter1).x, sub(points[i - 1], miter1).y);
        let (v4, v5) = (add(points[i - 1], miter1).x, add(points[i - 1], miter1).y);
        let (v6, v7) = (add(points[i - 1], miter1).x, add(points[i - 1], miter1).y);
        let (v8, v9) = (add(points[i], miter2).x, add(points[i], miter2).y);
        let (v10, v11) = (sub(points[i], miter2).x, sub(points[i], miter2).y);

        triangles.extend_from_slice(&[v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11]);
    }

    if points.len() > 2 {
        let n = points.len();
        let last_miter = scale(n2[0], width);

        let (v0, v1) = (sub(points[n - 1], last_miter).x, sub(points[n - 1], last_miter).y);
        let (v2, v3) = (sub(points[n - 2], miter1).x, sub(points[n - 2], miter1).y);
        let (v4, v5) = (add(points[n - 2], miter1).x, add(points[n - 2], miter1).y);
        let (v6, v7) = (add(points[n - 2], miter1).x, add(points[n - 2], miter1).y);
        let (v8, v9) = (add(points[n - 1], last_miter).x, add(points[n - 1], last_miter).y);
        let (v10, v11) = (sub(points[n - 1], last_miter).x, sub(points[n - 1], last_miter).y);

        triangles.extend_from_slice(&[v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11]);
    }

    triangles
}

fn calculate_normals(x: f64, y: f64) -> [Point; 2] {
    [
        normalize(Point { x: y, y: -x }),
        normalize(Point { x: -y, y: x }),
    ]
}

fn normalize(p: Point) -> Point {
    let norm = (p.x * p.x + p.y * p.y).sqrt();
    Point {
        x: p.x / norm,
        y: p.y / norm,
    }
}

fn scale(p: Point, f: f64) -> Point {
    Point {
        x: p.x * f,
        y: p.y * f,
    }
}

fn sub(p: Point, r: Point) -> Point {
    Point {
        x: p.x - r.x,
        y: p.y - r.y,
    }
}

fn add(p: Point, r: Point) -> Point {
    Point {
        x: p.x + r.x,
        y: p.y + r.y,
    }
}

fn dot(p: Point, r: Point) -> f64 {
    p.x * r.x + p.y * r.y
}
