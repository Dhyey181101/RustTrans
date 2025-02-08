



use std::cell::RefCell;
use std::collections::LinkedList;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 0.001 && (self.y - other.y).abs() < 0.001
    }
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
        (self.x - r.x) * (self.x - r.x) + (self.y - r.y) * (self.y - r.y)
    }
}

fn combine_polygons(outer: Vec<Point>, inner: Vec<Point>) -> Result<Vec<Point>, &'static str> {
    let x_max = inner.iter().map(|p| p.x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let m_index = inner.iter().position(|p| p.x > x_max).unwrap();
    let m = inner[m_index];

    let (k, k1, k2, err) = find_k(m, &outer)?;

    let mut visible_index = usize::MIN;
    for (i, p) in outer.iter().enumerate() {
        if *p == k {
            visible_index = i;
        }
    }

    let p_index = if outer[k1].x > outer[k2].x {
        k1
    } else {
        k2
    };

    let all_outside = are_all_outside(m, k, p_index, &outer);

    visible_index = if visible_index == usize::MIN && all_outside {
        p_index
    } else {
        find_closest(m, k, p_index, &outer)?
    };

    let mut result = vec![];
    result.extend_from_slice(&outer[..=visible_index]);
    for i in 0..inner.len() {
        let j = (m_index + i) % inner.len();
        result.push(inner[j]);
    }
    result.push(inner[m_index]);
    result.push(outer[visible_index]);
    result.extend_from_slice(&outer[visible_index + 1..]);

    Ok(result)
}

fn find_k(m: Point, outer: &[Point]) -> Result<(Point, usize, usize, f64), &'static str> {
    let mut err = "";
    let mut k = Point { x: f64::MIN, y: f64::MIN };
    let mut k1 = 0;
    let mut k2 = 0;

    for (i, j) in (0..outer.len()).zip((1..outer.len() + 1).rev()) {
        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = m.sub(outer[i]);
        let v2 = outer[j].sub(outer[i]);

        let t1 = v2.cross(v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < k.x {
                k = Point {
                    x: t1 + m.x,
                    y: m.y,
                };
                k1 = i;
                k2 = j;
            }
        } else {
            err = "cannot calculate intersection, problematic data";
        }
    }

    if err.is_empty() {
        Ok((k, k1, k2, 0.0))
    } else {
        Err(err)
    }
}

fn are_all_outside(m: Point, k: Point, p_index: usize, outer: &[Point]) -> bool {
    let mut all_outside = true;
    let p = outer[p_index];

    for i in 0..outer.len() {
        if i == p_index {
            continue;
        }

        if is_inside_triangle(m, k, p, outer[i]) {
            all_outside = false;
        }
    }

    all_outside
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    let ab = b.sub(a);
    let ac = c.sub(a);
    let ap = p.sub(a);

    let ab_cross_ac = ab.cross(ac);
    let ab_cross_ap = ab.cross(ap);
    let ac_cross_ap = ac.cross(ap);

    (ab_cross_ac > 0.0) == (ab_cross_ap > 0.0) && (ac_cross_ap > 0.0)
}

fn find_closest(
    m: Point,
    k: Point,
    p_index: usize,
    outer: &[Point],
) -> Result<usize, &'static str> {
    let mut reflex = RefCell::new(LinkedList::new());
    let n = outer.len();

    for i in 0..n {
        let not_inside = !is_inside_triangle(m, k, outer[p_index], outer[i]);
        let (prev, next) = (if i == 0 { n - 1 } else { i - 1 }, if i == n - 1 { 0 } else { i + 1 });
        let not_reflex = !is_reflex(outer[prev], outer[i], outer[next]);

        if not_inside || not_reflex {
            continue;
        }

        reflex.borrow_mut().push_back(i);
    }

    let mut closest = 0;
    let mut max_dist = f64::MIN;

    for r in reflex.borrow().iter() {
        let i = *r;
        let dist = outer[i].distance2(outer[closest]);

        if dist > max_dist {
            closest = i;
            max_dist = dist;
        }
    }

    Ok(closest)
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < -0.001
}



