
use std::f64::consts::PI;

#[derive(Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn combine_polygons(outer: Vec<Point>, inner: Vec<Point>) -> Result<Vec<Point>, String> {
    let mut x_max = 0.0;
    let mut m_index: usize = 0;
    for (i, point) in inner.iter().enumerate() {
        if point.x > x_max {
            x_max = point.x;
            m_index = i;
        }
    }

    let m = inner[m_index].clone();

    let (k, k1, k2) = find_k(&m, &outer).map_err(|_| "could not find visible vertex".to_string())?;

    let mut visible_index: isize = -1;
    for (i, point) in outer.iter().enumerate() {
        if eq_points(&point, &k) {
            visible_index = i as isize;
        }
    }

    let mut result = Vec::with_capacity(outer.len() + inner.len() + 2);
    result.extend_from_slice(&outer[..=visible_index as usize]);
    for i in 0..inner.len() {
        result.push(inner[(m_index + i) % inner.len()].clone());
    }
    result.push(m.clone());
    result.push(outer[visible_index as usize].clone());
    result.extend_from_slice(&outer[(visible_index as usize + 1)..]);

    Ok(result)
}

fn find_k(m: &Point, outer: &[Point]) -> Result<(Point, usize, usize), ()> {
    let mut k = Point { x: 0.0, y: 0.0 };
    let mut k1 = 0;
    let mut k2 = 0;

    for (i, j) in outer.windows(2).enumerate() {
        if j[0].y > m.y || j[1].y < m.y {
            continue;
        }

        let v1 = sub_points(m.clone(), j[0].clone());
        let v2 = sub_points(j[1].clone(), j[0].clone());

        let t1 = cross_points(&v1, v2.clone()) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < k.x {
                k = Point {
                    x: t1 + m.x,
                    y: m.y,
                };
                k1 = i;
                k2 = i + 1;
                return Ok((k, k1, k2));
            }
        } else {
            return Err(());
        }
    }

    Err(())
}

fn sub_points(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}

fn cross_points(p1: &Point, p2: Point) -> f64 {
    p1.x * p2.y - p1.y * p2.x
}

fn dist_squared(p1: Point, p2: Point) -> f64 {
    (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)
}

fn eq_points(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x && p1.y == p2.y
}

