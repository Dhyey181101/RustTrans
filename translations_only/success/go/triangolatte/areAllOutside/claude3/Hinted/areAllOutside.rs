
struct Point {
    x: f64,
    y: f64,
}

fn are_all_outside(m: &Point, k: &Box<Point>, p_index: usize, outer: &Vec<Point>) -> bool {
    let mut all_outside = true;
    for i in 0..outer.len() {
        if i == p_index {
            continue;
        }

        if is_inside_triangle(&m, k.as_ref(), &outer[p_index], &outer[i]) {
            all_outside = false;
        }
    }
    all_outside
}

fn is_inside_triangle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0
        && (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0
        && (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}
