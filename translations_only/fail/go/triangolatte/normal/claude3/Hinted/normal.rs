
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn normal(points: &[Point], width: f64) -> Vec<f64> {
    let mut triangles = Vec::with_capacity(points.len() * 12);
    let width = width / 2.0;

    for i in 0..points.len() - 1 {
        let dx = points[i + 1].x - points[i].x;
        let dy = points[i + 1].y - points[i].y;
        let n1 = scale(Point { x: dy, y: -dx }, width);
        let n2 = scale(Point { x: -dy, y: dx }, width);

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

fn add(p: Point, r: Point) -> Point {
    Point {
        x: p.x + r.x,
        y: p.y + r.y,
    }
}
