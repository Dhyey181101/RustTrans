

use std::cell::RefCell;
use std::collections::LinkedList;
use std::f64;

const EPSILON: f64 = 0.001;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn sub(&self, r: Point) -> Point {
        Point {
            x: self.x - r.x,
            y: self.y - r.y,
        }
    }

    fn cross(&self, r: Point) -> f64 {
        self.x * r.y - self.y * r.x
    }

    fn distance2(&self, r: Point) -> f64 {
        (self.x - r.x).powf(2.0) + (self.y - r.y).powf(2.0)
    }
}

fn combine_polygons(
    outer: Vec<Point>,
    inner: Vec<Point>,
) -> Result<Vec<Point>, &'static str> {
    let x_max = inner.iter().map(|p| p.x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let m_index = inner.iter().position(|p| p.x > x_max).unwrap();
    let m = inner[m_index];

    let (k, k1, k2, err) = find_k(m, &outer)?;

    let mut visible_index = 0;
    for (i, p) in outer.iter().enumerate() {
        if *p == k {
            visible_index = i;
            break;
        }
    }

    let p_index = if outer[k1].x > outer[k2].x {
        k1
    } else {
        k2
    };

    let all_outside = are_all_outside(m, k, p_index, &outer);

    visible_index = if visible_index == 0 && all_outside {
        p_index
    } else {
        find_closest(m, k, p_index, &outer)?
    };

    let mut result = vec![];
    result.extend_from_slice(&outer[..=visible_index]);
    result.extend(inner.iter().cloned());
    result.push(inner[m_index]);
    result.push(outer[visible_index]);
    result.extend_from_slice(&outer[visible_index + 1..]);

    Ok(result)
}

fn find_k(
    m: Point,
    outer: &[Point],
) -> Result<(Point, usize, usize, f64), &'static str> {
    let n = outer.len();
    let mut k = Point { x: f64::INFINITY, y: 0.0 };
    let mut k1 = 0;
    let mut k2 = 0;
    let mut err = 0.0;

    for (i, p) in outer.iter().enumerate().rev() {
        let q = outer[(i + 1) % n];

        if p.y <= m.y || q.y > m.y {
            continue;
        }

        let t1 = (q.x - p.x) * (m.y - p.y) / (q.y - p.y) + p.x;

        if t1 >= m.x - EPSILON && t1 <= m.x + EPSILON && t1 > k.x {
            k = Point { x: t1, y: m.y };
            k1 = i;
            k2 = (i + 1) % n;
            err = 0.0;
        }
    }

    if err > 0.0 {
        Err("cannot calculate intersection, problematic data")
    } else {
        Ok((k, k1, k2, err))
    }
}

fn are_all_outside(
    m: Point,
    k: Point,
    p_index: usize,
    outer: &[Point],
) -> bool {
    let mut all_outside = true;
    let p = &outer[p_index];

    for i in 0..outer.len() {
        if i == p_index {
            continue;
        }

        if is_inside_triangle(m, k, outer[p_index], &outer[i]) {
            all_outside = false;
        }
    }

    all_outside
}

fn is_inside_triangle(
    a: Point,
    b: Point,
    c: Point,
    p: &Point,
) -> bool {
    let ab = b.sub(a);
    let ac = c.sub(a);
    let ap = p.sub(a);

    let ab_cross_ac = ab.cross(ac);
    let ab_cross_ap = ab.cross(ap);
    let ac_cross_ap = ac.cross(ap);

    (ab_cross_ac > 0.0) == (ab_cross_ap > 0.0) && (ab_cross_ac > 0.0) == (ac_cross_ap > 0.0)
}

fn find_closest(
    m: Point,
    k: Point,
    p_index: usize,
    outer: &[Point],
) -> Result<usize, &'static str> {
    let mut min_distance = f64::INFINITY;
    let mut closest_index = 0;

    for (i, p) in outer.iter().enumerate() {
        if i == p_index {
            continue;
        }

        if is_inside_triangle(m, k, outer[p_index], p) {
            let distance = m.distance2(*p);
            if distance < min_distance {
                min_distance = distance;
                closest_index = i;
            }
        }
    }

    if min_distance == f64::INFINITY {
        Err("no point found")
    } else {
        Ok(closest_index)
    }
}

