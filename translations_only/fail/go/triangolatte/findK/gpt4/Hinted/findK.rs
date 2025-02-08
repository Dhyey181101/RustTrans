
struct Point {
    x: f64,
    y: f64,
}

fn sub(p: &Point, r: &Point) -> Point {
    Point {
        x: p.x - r.x,
        y: p.y - r.y,
    }
}

fn cross(p: &Point, r: &Point) -> f64 {
    p.x * r.y - p.y * r.x
}

fn find_k(m: Point, outer: Vec<Point>) -> (Point, i32, i32, Option<Box<dyn std::error::Error>>) {
    let mut k = Point { x: f64::INFINITY, y: m.y };
    let mut k1 = 0;
    let mut k2 = 0;
    let mut err: Option<Box<dyn std::error::Error>> = None;

    for (i, j) in (0..outer.len()).zip(1..=outer.len()) {
        let i = if i == outer.len() { 0 } else { i };
        if outer[i].y > m.y || outer[j % outer.len()].y < m.y {
            continue;
        }

        let v1 = sub(&m, &outer[i]);
        let v2 = sub(&outer[j % outer.len()], &outer[i]);

        if v2.y == 0.0 {
            err = Some(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "cannot calculate intersection, problematic data",
            )));
            break;
        }

        let t1 = cross(&v2, &v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 < k.x - m.x {
                k = Point { x: t1 + m.x, y: m.y };
                k1 = i as i32;
                k2 = j as i32;
            }
        } else {
            err = Some(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "cannot calculate intersection, problematic data",
            )));
            break;
        }
    }

    (k, k1, k2, err)
}
