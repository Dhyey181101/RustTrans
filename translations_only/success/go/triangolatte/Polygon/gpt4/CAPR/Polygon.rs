
use std::error::Error;
use std::fmt;

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}

struct Element {
    point: Point,
    prev: usize,
    next: usize,
}

#[derive(Debug, Clone)]
struct PolygonError {
    details: String,
}

impl PolygonError {
    fn new(msg: &str) -> PolygonError {
        PolygonError { details: msg.to_string() }
    }
}

impl fmt::Display for PolygonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PolygonError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn polygon(points: Vec<Point>) -> Result<Vec<f64>, Box<dyn Error>> {
    let n = points.len();

    if n < 3 {
        return Err(Box::new(PolygonError::new("cannot triangulate less than three points")));
    }

    let mut elements = Vec::with_capacity(n);
    for i in 0..n {
        elements.push(Element {
            point: points[i].clone(),
            prev: if i == 0 { n - 1 } else { i - 1 },
            next: if i == n - 1 { 0 } else { i + 1 },
        });
    }

    let mut ear = 0;
    let mut t = vec![0.0; (n - 2) * 6];
    let mut i = 0;
    let mut stop = ear;
    let mut iterations = 0;

    while elements[ear].prev != elements[ear].next {
        let prev = elements[ear].prev;
        let next = elements[ear].next;

        if is_ear(&elements, ear) {
            let prev_point = elements[prev].point.clone();
            let ear_point = elements[ear].point.clone();
            let next_point = elements[next].point.clone();

            if polygon_area(&vec![prev_point.clone(), ear_point.clone(), next_point.clone()]) > 0.0 {
                t[i] = prev_point.x;
                t[i + 1] = prev_point.y;
                t[i + 2] = ear_point.x;
                t[i + 3] = ear_point.y;
                t[i + 4] = next_point.x;
                t[i + 5] = next_point.y;
                i += 6;
            }

            remove(&mut elements, ear);
            ear = next;
            stop = elements[stop].next;
            continue;
        }

        ear = next;

        if ear == stop || iterations > n * 2 { // Prevent infinite loop
            return Err(Box::new(PolygonError::new("oops")));
        }
        iterations += 1;
    }

    Ok(t.into_iter().take(i).collect())
}

fn is_ear(elements: &Vec<Element>, index: usize) -> bool {
    let a = elements[elements[index].prev].point.clone();
    let b = elements[index].point.clone();
    let c = elements[elements[index].next].point.clone();
    if is_reflex(&a, &b, &c) {
        return false;
    }

    let mut r = elements[index].next;
    while r != elements[index].prev {
        let inside = is_inside_triangle(&a, &b, &c, &elements[r].point);
        let reflex = is_reflex(&elements[elements[r].prev].point, &elements[r].point, &elements[elements[r].next].point);
        if inside && reflex {
            return false;
        }
        r = elements[r].next;
    }
    true
}

fn is_reflex(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    let cross1 = (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0;
    let cross2 = (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0;
    let cross3 = (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0;
    cross1 && cross2 && cross3
}

fn polygon_area(points: &Vec<Point>) -> f64 {
    let mut area = 0.0;
    let n = points.len();
    for i in 0..n {
        let j = if i == 0 { n - 1 } else { i - 1 };
        area += points[i].x * points[j].y - points[i].y * points[j].x;
    }
    area.abs() / 2.0
}

fn remove(elements: &mut Vec<Element>, index: usize) {
    let prev = elements[index].prev;
    let next = elements[index].next;
    elements[prev].next = next;
    elements[next].prev = prev;
}
