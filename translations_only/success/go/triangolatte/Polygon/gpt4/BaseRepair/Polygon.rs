
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

    let mut elements = Vec::with_capacity(n);
    for (i, point) in points.iter().enumerate() {
        elements.push(Element {
            prev: if i == 0 { n - 1 } else { i - 1 },
            next: if i == n - 1 { 0 } else { i + 1 },
            point: *point,
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
            let prev_point = elements[prev].point;
            let ear_point = elements[ear].point;
            let next_point = elements[next].point;

            if polygon_area(&vec![prev_point, ear_point, next_point]) > 0.0 {
                t[i..i + 6].clone_from_slice(&[
                    prev_point.x, prev_point.y,
                    ear_point.x, ear_point.y,
                    next_point.x, next_point.y,
                ]);
                i += 6;
            }

            remove(&mut elements, ear);
            ear = next;
            stop = elements[stop].next;
            continue;
        }

        ear = next;

        if ear == stop {
            if iterations > n * 2 {
                return Err("oops".into());
            }
            iterations += 1;
        }
    }

    Ok(t[0..i].to_vec())
}

fn is_ear(elements: &Vec<Element>, index: usize) -> bool {
    let a = elements[elements[index].prev].point;
    let b = elements[index].point;
    let c = elements[elements[index].next].point;
    if is_reflex(a, b, c) {
        return false;
    }

    let mut r = elements[elements[index].next].next;
    while r != elements[index].prev {
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
    let ab = (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0;
    let bc = (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0;
    let ca = (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0;
    ab && bc && ca
}

fn polygon_area(data: &Vec<Point>) -> f64 {
    let mut area = 0.0;
    for i in 0..data.len() {
        let j = if i == 0 { data.len() - 1 } else { i - 1 };
        area += data[i].x * data[j].y - data[i].y * data[j].x;
    }
    area.abs() / 2.0
}

fn remove(elements: &mut Vec<Element>, index: usize) {
    let prev = elements[index].prev;
    let next = elements[index].next;
    elements[prev].next = next;
    elements[next].prev = prev;
}
