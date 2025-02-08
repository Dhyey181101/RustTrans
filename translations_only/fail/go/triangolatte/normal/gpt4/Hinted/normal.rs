
struct Point {
    x: f64,
    y: f64,
}

fn normal(points: Vec<Point>, mut width: f64) -> Vec<f64> {
    width /= 2.0;
    let mut triangles = Vec::with_capacity(points.len() * 12);
    for i in 0..=points.len() - 2 {
        let dx = points[i + 1].x - points[i].x;
        let dy = points[i + 1].y - points[i].y;
        let n1 = scale(Point { x: dy, y: -dx }, width);
        let n2 = scale(Point { x: -dy, y: dx }, width);

        let (v0, v1) = add(&points[i + 1], &n2);
        let (v2, v3) = add(&points[i], &n2);
        let (v4, v5) = add(&points[i], &n1);
        let (v6, v7) = add(&points[i], &n1);
        let (v8, v9) = add(&points[i + 1], &n1);
        let (v10, v11) = add(&points[i + 1], &n2);

        triangles.extend_from_slice(&[v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11]);
    }

    triangles
}

fn scale(p: Point, f: f64) -> Point {
    let norm = f64::sqrt(p.x * p.x + p.y * p.y);
    Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    }
}

fn add(p: &Point, r: &Point) -> (f64, f64) {
    (p.x + r.x, p.y + r.y)
}
