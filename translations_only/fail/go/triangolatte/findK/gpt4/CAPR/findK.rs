
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

fn find_k(m: &Point, outer: &[Point]) -> Result<(Point, usize, usize), Box<dyn std::error::Error>> {
    let mut k = Point { x: f64::INFINITY, y: m.y };
    let mut k1 = 0;
    let mut k2 = 0;
    let mut found = false;

    for i in 0..outer.len() {
        let j = (i + 1) % outer.len();

        // Skip edges that do not have their first point below `m` and the second one above.
        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = sub(m, &outer[i]);
        let v2 = sub(&outer[j], &outer[i]);

        let t1 = cross(&v2, &v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            // If there is no current `k` candidate or this one is closer.
            if t1 < k.x - m.x {
                k = Point { x: t1 + m.x, y: m.y };
                k1 = i;
                k2 = j;
                found = true;
            }
        } else {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "cannot calculate intersection, problematic data")));
        }
    }

    if found {
        Ok((k, k1, k2))
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "no intersection found")))
    }
}
