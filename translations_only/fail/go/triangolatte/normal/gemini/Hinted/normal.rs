
use std::f64::consts::PI;

fn normal(points: &[Point], width: f64) -> Vec<f64> {
    let width = width / 2.0;
    let mut triangles = Vec::with_capacity(points.len() * 12);
    for i in 0..points.len() - 1 {
        let dx = points[i + 1].x - points[i].x;
        let dy = points[i + 1].y - points[i].y;
        let n1 = Point {
            x: dy,
            y: -dx,
        }
        .scale(width);
        let n2 = Point {
            x: -dy,
            y: dx,
        }
        .scale(width);

        let (v0, v1) = points[i + 1].add(n2);
        let (v2, v3) = points[i].add(n2);
        let (v4, v5) = points[i].add(n1);
        let (v6, v7) = points[i].add(n1);
        let (v8, v9) = points[i + 1].add(n1);
        let (v10, v11) = points[i + 1].add(n2);

        triangles.extend_from_slice(&[v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11]);
    }

    triangles
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn scale(self, f: f64) -> Self {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        Self {
            x: self.x / norm * f,
            y: self.y / norm * f,
        }
    }

    fn add(self, r: Point) -> (f64, f64) {
        (self.x + r.x, self.y + r.y)
    }
}
