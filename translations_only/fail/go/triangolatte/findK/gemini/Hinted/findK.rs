
use std::f64::consts::PI;

fn find_k(m: Point, outer: &[Point]) -> Result<(Point, usize, usize), String> {
    for (i, j) in (0..outer.len()).rev().zip(0..outer.len()) {
        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = sub(m, outer[i]);
        let v2 = sub(outer[j], outer[i]);

        let t1 = cross(v2, v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < outer[i].x {
                return Ok((Point { x: t1 + m.x, y: m.y }, i, j));
            }
        } else {
            return Err("cannot calculate intersection, problematic data".to_string());
        }
    }
    Err("no intersection found".to_string())
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn sub(a: Point, b: Point) -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

fn cross(a: Point, b: Point) -> f64 {
    a.x * b.y - a.y * b.x
}
