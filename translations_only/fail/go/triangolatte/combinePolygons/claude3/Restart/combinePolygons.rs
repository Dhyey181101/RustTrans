
use std::collections::LinkedList;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn sub(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}

fn cross(p1: Point, p2: Point) -> f64 {
    p1.x * p2.y - p1.y * p2.x
}

fn distance2(p1: Point, p2: Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    dx * dx + dy * dy
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0
        && (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0
        && (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn cyclic(i: usize, n: usize) -> usize {
    (i % n + n) % n
}

fn are_all_outside(m: Point, k: Point, p_index: usize, outer: &[Point]) -> bool {
    let mut all_outside = true;
    for i in 0..outer.len() {
        if i == p_index {
            continue;
        }
        if is_inside_triangle(m, k, outer[p_index], outer[i]) {
            all_outside = false;
            break;
        }
    }
    all_outside
}

fn find_closest(m: Point, k: Point, p_index: usize, outer: &[Point]) -> usize {
    let mut reflex = LinkedList::new();
    let n = outer.len();
    for i in 0..n {
        let not_inside = !is_inside_triangle(m, k, outer[p_index], outer[i]);
        let prev = cyclic(i.wrapping_sub(1), n);
        let next = cyclic(i + 1, n);
        let not_reflex = !is_reflex(outer[prev], outer[i], outer[next]);
        if not_inside || not_reflex {
            continue;
        }
        reflex.push_back(i);
    }

    let mut closest = 0;
    let mut max_dist = 0.0;

    for i in reflex.iter() {
        let dist = distance2(outer[*i], outer[closest]);
        if dist > max_dist {
            closest = *i;
            max_dist = dist;
        }
    }

    closest
}

fn find_k(m: Point, outer: &[Point]) -> (Point, Box<usize>, Box<usize>) {
    let mut k = Point { x: 0.0, y: 0.0 };
    let mut k1 = Box::new(0);
    let mut k2 = Box::new(0);

    for (i, j) in outer.iter().zip(outer.iter().skip(1).chain(outer.iter().take(1))) {
        if j.y > m.y || i.y < m.y {
            continue;
        }

        let v1 = sub(m, *i);
        let v2 = sub(*j, *i);

        let t1 = cross(v2, v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < k.x {
                k = Point {
                    x: t1 + m.x,
                    y: m.y,
                };
                *k1 = outer.iter().position(|p| p == i).unwrap();
                *k2 = outer.iter().position(|p| p == j).unwrap();
            }
        }
    }

    (k, k1, k2)
}

fn combine_polygons(outer: &[Point], inner: &[Point]) -> Result<Vec<Point>, &'static str> {
    let mut x_max = 0.0;
    let mut m_index = 0;
    for (i, p) in inner.iter().enumerate() {
        if p.x > x_max {
            x_max = p.x;
            m_index = i;
        }
    }

    let m = inner[m_index];

    let mut visible_index = Box::new(0);
    let (k, k1, k2) = find_k(m, outer);

    for (i, p) in outer.iter().enumerate() {
        if *p == k {
            *visible_index = i;
            break;
        }
    }

    let p_index = if outer[*k1].x > outer[*k2].x {
        *k1
    } else {
        *k2
    };

    let all_outside = are_all_outside(m, k, p_index, outer);

    if *visible_index == 0 && all_outside {
        *visible_index = p_index;
    }

    if *visible_index == 0 {
        *visible_index = find_closest(m, k, p_index, outer);
    }

    if *visible_index == 0 {
        return Err("could not find visible vertex");
    }

    let mut result = Vec::with_capacity(outer.len() + inner.len() + 2);
    result.extend_from_slice(&outer[..*visible_index + 1]);
    for i in 0..inner.len() {
        result.push(inner[cyclic(m_index + i, inner.len())]);
    }
    result.push(inner[m_index]);
    result.push(outer[*visible_index]);
    result.extend_from_slice(&outer[*visible_index + 1..]);

    Ok(result)
}
