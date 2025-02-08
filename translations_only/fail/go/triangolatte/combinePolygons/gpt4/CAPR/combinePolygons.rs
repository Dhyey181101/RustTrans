
use std::collections::LinkedList;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    fn distance2(&self, other: &Point) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

#[derive(Debug, Clone)]
struct CustomError {
    details: String,
}

impl CustomError {
    fn new(msg: &str) -> CustomError {
        CustomError { details: msg.to_string() }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn combine_polygons(outer: Vec<Point>, inner: Vec<Point>) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut x_max = 0.0;
    let mut m_index = 0;
    for (i, point) in inner.iter().enumerate() {
        if point.x > x_max {
            x_max = point.x;
            m_index = i;
        }
    }

    let m = inner[m_index].clone();

    let mut p_index = 0;
    let mut visible_index = -1;

    let (k, k1, k2, err) = find_k(&m, &outer)?;

    for (i, point) in outer.iter().enumerate() {
        if *point == k {
            visible_index = i as isize;
        }
    }

    if outer[k1].x > outer[k2].x {
        p_index = k1;
    } else {
        p_index = k2;
    }

    let all_outside = are_all_outside(&m, &k, p_index, &outer);

    if visible_index < 0 && all_outside {
        visible_index = p_index as isize;
    }

    if visible_index < 0 {
        visible_index = find_closest(&m, &k, p_index, &outer) as isize;
    }

    if visible_index < 0 {
        return Err(Box::new(CustomError::new("could not find visible vertex")));
    }

    let mut result = Vec::with_capacity(outer.len() + inner.len() + 2);
    result.extend_from_slice(&outer[..=visible_index as usize]);
    for i in 0..inner.len() {
        result.push(inner[cyclic(m_index as isize + i as isize, inner.len())].clone());
    }
    result.push(inner[m_index].clone());
    result.push(outer[visible_index as usize].clone());
    result.extend_from_slice(&outer[(visible_index as usize + 1)..]);

    Ok(result)
}

fn find_k(m: &Point, outer: &[Point]) -> Result<(Point, usize, usize, Box<dyn Error>), Box<dyn Error>> {
    for (i, j) in (0..outer.len()).zip(1..=outer.len()) {
        let i = if i == 0 { outer.len() - 1 } else { i - 1 };
        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = m.sub(&outer[i]);
        let v2 = outer[j].sub(&outer[i]);

        let t1 = v2.cross(&v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            let k = Point { x: t1 + m.x, y: m.y };
            return Ok((k, i, j, Box::new(CustomError::new(""))));
        } else {
            return Err(Box::new(CustomError::new("cannot calculate intersection, problematic data")));
        }
    }
    Err(Box::new(CustomError::new("no intersection found")))
}

fn are_all_outside(m: &Point, k: &Point, p_index: usize, outer: &[Point]) -> bool {
    outer.iter().enumerate().all(|(i, point)| {
        if i == p_index {
            true
        } else {
            !is_inside_triangle(m, k, &outer[p_index], point)
        }
    })
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    ((c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0)
        && ((a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0)
        && ((b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0)
}

fn find_closest(m: &Point, k: &Point, p_index: usize, outer: &[Point]) -> usize {
    let mut reflex = LinkedList::new();
    let n = outer.len();
    for i in 0..n {
        let not_inside = !is_inside_triangle(m, k, &outer[p_index], &outer[i]);
        let prev = cyclic(i as isize - 1, n);
        let next = cyclic(i as isize + 1, n);
        let not_reflex = !is_reflex(&outer[prev], &outer[i], &outer[next]);
        if not_inside || not_reflex {
            continue;
        }
        reflex.push_back(i);
    }
    let mut closest = 0;
    let mut max_dist = 0.0;

    for &i in reflex.iter() {
        let dist = outer[i].distance2(&outer[closest]);
        if dist > max_dist {
            closest = i;
            max_dist = dist;
        }
    }
    closest
}

fn cyclic(i: isize, n: usize) -> usize {
    ((i % n as isize + n as isize) % n as isize) as usize
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}
