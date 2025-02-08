

use std::collections::LinkedList;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn sub(p: Point, r: Point) -> Point {
    Point {
        x: p.x - r.x,
        y: p.y - r.y,
    }
}

fn cross(p: Point, r: Point) -> f64 {
    p.x * r.y - p.y * r.x
}

fn combine_polygons(outer: &[Point], inner: &[Point]) -> Result<Vec<Point>, &'static str> {
    let mut x_max = 0.0;
    let mut m_index = 0;
    for i in 0..inner.len() {
        if inner[i].x > x_max {
            x_max = inner[i].x;
            m_index = i;
        }
    }

    let m = inner[m_index];

    let (mut k, mut k1, mut k2) = (Point { x: 0.0, y: 0.0 }, 0, 0);
    if let Err(err) = find_k(m, outer, &mut k, &mut k1, &mut k2) {
        return Err(err);
    }

    let mut visible_index = -1;

    for i in 0..outer.len() {
        if outer[i] == k {
            visible_index = i as isize;
        }
    }

    let p_index = if outer[k1].x > outer[k2].x {
        k1
    } else {
        k2
    };

    let all_outside = are_all_outside(m, k, p_index, outer);

    if visible_index < 0 && all_outside {
        visible_index = p_index as isize;
    }

    if visible_index < 0 {
        visible_index = find_closest(m, k, p_index, outer) as isize;
    }

    if visible_index < 0 {
        return Err("could not find visible vertex");
    }

    let mut result = Vec::with_capacity(outer.len() + inner.len() + 2);
    result.extend_from_slice(&outer[..=(visible_index as usize)]);
    for i in 0..inner.len() {
        result.push(inner[cyclic(m_index as isize + i as isize, inner.len() as isize) as usize]);
    }
    result.push(inner[m_index]);
    result.push(outer[visible_index as usize]);
    result.extend_from_slice(&outer[(visible_index as usize + 1)..]);

    Ok(result)
}

fn find_k(
    m: Point,
    outer: &[Point],
    k: &mut Point,
    k1: &mut usize,
    k2: &mut usize,
) -> Result<(), &'static str> {
    for (i, j) in (0..outer.len()).zip(1..outer.len()).chain(Some((outer.len() - 1, 0))) {
        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = sub(m, outer[i]);
        let v2 = sub(outer[j], outer[i]);

        let t1 = cross(v2, v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < k.x {
                k.x = t1 + m.x;
                k.y = m.y;
                *k1 = i;
                *k2 = j;
                return Ok(());
            }
        } else {
            return Err("cannot calculate intersection, problematic data");
        }
    }
    Ok(())
}

fn are_all_outside(m: Point, k: Point, p_index: usize, outer: &[Point]) -> bool {
    let mut all_outside = true;
    for i in 0..outer.len() {
        if i == p_index {
            continue;
        }

        if is_inside_triangle(m, k, outer[p_index], outer[i]) {
            all_outside = false;
        }
    }
    all_outside
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0
        && (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0
        && (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

fn find_closest(m: Point, k: Point, p_index: usize, outer: &[Point]) -> usize {
    let mut reflex = LinkedList::new();
    let n = outer.len();
    for i in 0..n {
        let not_inside = !is_inside_triangle(m, k, outer[p_index], outer[i]);
        let prev = cyclic(i as isize - 1, n as isize) as usize;
        let next = cyclic(i as isize + 1, n as isize) as usize;
        let not_reflex = !is_reflex(outer[prev], outer[i], outer[next]);
        if not_inside || not_reflex {
            continue;
        }
        reflex.push_back(i);
    }
    let mut closest = 0;
    let mut max_dist = 0.0;

    for r in reflex.iter() {
        let i = *r;
        let dist = distance2(outer[i], outer[closest]);
        if dist > max_dist {
            closest = i;
            max_dist = dist;
        }
    }
    closest
}

fn cyclic(i: isize, n: isize) -> isize {
    ((i % n + n) % n)
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn distance2(p: Point, r: Point) -> f64 {
    (p.x - r.x) * (p.x - r.x) + (p.y - r.y) * (p.y - r.y)
}

