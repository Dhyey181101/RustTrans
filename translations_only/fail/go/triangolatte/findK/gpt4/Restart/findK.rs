
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn sub(&self, r: &Point) -> Point {
        Point {
            x: self.x - r.x,
            y: self.y - r.y,
        }
    }

    fn cross(&self, r: &Point) -> f64 {
        self.x * r.y - self.y * r.x
    }
}

fn find_k(m: &Point, outer: &[Point]) -> Result<(Point, usize, usize), Box<dyn std::error::Error>> {
    let mut k = Point { x: f64::MAX, y: m.y };
    let mut k1 = 0;
    let mut k2 = 0;
    let mut found = false;

    for (i, j) in (0..outer.len()).zip(1..=outer.len()) {
        let i = if i == 0 { outer.len() - 1 } else { i - 1 };

        if outer[i].y > m.y || outer[j % outer.len()].y < m.y {
            continue;
        }

        let v1 = m.sub(&outer[i]);
        let v2 = outer[j % outer.len()].sub(&outer[i]);

        let t1 = v2.cross(&v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 < k.x - m.x {
                k = Point { x: t1 + m.x, y: m.y };
                k1 = i;
                k2 = j % outer.len();
                found = true;
            }
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "cannot calculate intersection, problematic data",
            )));
        }
    }

    if found {
        Ok((k, k1, k2))
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no intersection found",
        )))
    }
}
