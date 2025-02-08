

#[derive(Clone, PartialEq)]
struct Element {
    prev: Box<Element>,
    next: Box<Element>,
    point: Point,
}

fn polygon(points: &[Point]) -> Result<Vec<f64>, &'static str> {
    let n = points.len();

    if n < 3 {
        return Err("cannot triangulate less than three points");
    }

    let mut elements: Vec<Box<Element>> = Vec::with_capacity(n);
    for i in 0..n {
        let prev = Box::new(Element {
            prev: if i == 0 {
                elements[n - 1].clone()
            } else {
                elements[i - 1].clone()
            },
            next: if i == n - 1 {
                elements[0].clone()
            } else {
                elements[i + 1].clone()
            },
            point: points[i],
        });
        elements.push(prev);
    }

    let mut ear = elements[0].clone();
    let mut i = 0;
    let mut t = Vec::with_capacity((n - 2) * 6);

    let mut stop = ear.clone();
    let mut prev;
    let mut next_elem;

    while ear.prev.next != ear.prev.prev.next {
        prev = ear.prev.clone();
        next_elem = ear.next.clone();

        if is_ear(&prev, &ear, &next_elem) {
            if polygon_area(&[prev.point, ear.point, next_elem.point]) > 0.0 {
                t.push(prev.point.x);
                t.push(prev.point.y);
                t.push(ear.point.x);
                t.push(ear.point.y);
                t.push(next_elem.point.x);
                t.push(next_elem.point.y);
                i += 6;
            }

            ear.prev.next = next_elem.clone();
            ear.next.prev = prev.clone();
            ear = next_elem;
            stop = stop.next.clone();
            continue;
        }

        ear = ear.next.clone();

        if ear == stop {
            return Err("oops");
        }
    }

    Ok(t)
}

fn is_ear(prev: &Element, ear: &Element, next: &Element) -> bool {
    let a = prev.point;
    let b = ear.point;
    let c = next.point;
    if is_reflex(a, b, c) {
        return false;
    }

    let mut r = next.next.clone();
    while r != prev.prev.next {
        if is_inside_triangle(a, b, c, r.point) && is_reflex(r.prev.point, r.point, r.next.point) {
            return false;
        }
        r = r.next.clone();
    }
    true
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0
        && (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0
        && (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

fn polygon_area(data: &[Point]) -> f64 {
    let mut area = 0.0;
    for (i, p) in data.iter().enumerate() {
        let j = if i == data.len() - 1 { 0 } else { i + 1 };
        area += p.x * data[j].y - p.y * data[j].x;
    }
    area.abs() / 2.0
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

