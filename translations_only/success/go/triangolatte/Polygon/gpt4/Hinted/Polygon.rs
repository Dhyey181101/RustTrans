
use std::error::Error;
use std::f64;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

struct Element {
    prev: usize,
    next: usize,
    point: Point,
}

fn polygon(points: Vec<Point>) -> Result<Vec<f64>, Box<dyn Error>> {
    let n = points.len();

    if n < 3 {
        return Err("cannot triangulate less than three points".into());
    }

    let mut elements: Vec<Element> = Vec::with_capacity(n);
    elements.push(Element {
        prev: n - 1,
        next: 1,
        point: points[0],
    });
    for i in 1..n - 1 {
        elements.push(Element {
            prev: i - 1,
            next: i + 1,
            point: points[i],
        });
    }
    elements.push(Element {
        prev: n - 2,
        next: 0,
        point: points[n - 1],
    });

    let mut ear = 0;
    let mut t: Vec<f64> = vec![0.0; (n - 2) * 6];
    let mut i = 0;

    let mut stop = ear;
    let mut prev;
    let mut next;

    while elements[ear].prev != elements[ear].next {
        prev = elements[ear].prev;
        next = elements[ear].next;

        if is_ear(&elements, ear) {
            if polygon_area(vec![elements[prev].point, elements[ear].point, elements[next].point]) > 0.0 {
                t[i] = elements[prev].point.x;
                t[i + 1] = elements[prev].point.y;
                t[i + 2] = elements[ear].point.x;
                t[i + 3] = elements[ear].point.y;
                t[i + 4] = elements[next].point.x;
                t[i + 5] = elements[next].point.y;
                i += 6;
            }

            remove(&mut elements, ear);
            ear = next;
            stop = elements[stop].next;
            continue;
        }

        ear = next;

        if ear == stop {
            return Err("oops".into());
        }
    }

    t.truncate(i);
    Ok(t)
}

fn is_ear(elements: &Vec<Element>, p: usize) -> bool {
    let a = elements[elements[p].prev].point;
    let b = elements[p].point;
    let c = elements[elements[p].next].point;
    if is_reflex(a, b, c) {
        return false;
    }

    let mut r = elements[elements[p].next].next;
    while r != elements[p].prev {
        let inside = is_inside_triangle(a, b, c, elements[r].point);
        let reflex = is_reflex(elements[elements[r].prev].point, elements[r].point, elements[elements[r].next].point);
        if inside && reflex {
            return false;
        }
        r = elements[r].next;
    }
    true
}

fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) < 0.0
}

fn is_inside_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0 &&
    (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0 &&
    (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

fn polygon_area(data: Vec<Point>) -> f64 {
    let mut area = 0.0;
    let mut j = data.len() - 1;
    for i in 0..data.len() {
        area += data[i].x * data[j].y - data[i].y * data[j].x;
        j = i;
    }
    area.abs() / 2.0
}

fn remove(elements: &mut Vec<Element>, index: usize) {
    let prev = elements[index].prev;
    let next = elements[index].next;
    elements[prev].next = next;
    elements[next].prev = prev;
}
