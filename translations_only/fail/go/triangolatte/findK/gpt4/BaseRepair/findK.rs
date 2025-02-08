
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
}

fn find_k(m: &Point, outer: &[Point]) -> (Option<Point>, Option<usize>, Option<usize>, Option<String>) {
    let mut k: Option<Point> = None;
    let mut k1: Option<usize> = None;
    let mut k2: Option<usize> = None;
    let mut err: Option<String> = None;

    let mut i = outer.len() - 1;
    let mut j = 0;
    while j < outer.len() {
        if outer[i].y > m.y || outer[j].y < m.y {
            i = j;
            j += 1;
            continue;
        }

        let v1 = m.sub(&outer[i]);
        let v2 = outer[j].sub(&outer[i]);

        let t1 = v2.cross(&v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            match k {
                Some(ref k_val) if t1 - m.x < k_val.x => {
                    k = Some(Point { x: t1 + m.x, y: m.y });
                    k1 = Some(i);
                    k2 = Some(j);
                }
                None => {
                    k = Some(Point { x: t1 + m.x, y: m.y });
                    k1 = Some(i);
                    k2 = Some(j);
                }
                _ => {}
            }
        } else {
            err = Some("cannot calculate intersection, problematic data".to_string());
            return (k, k1, k2, err);
        }

        i = j;
        j += 1;
    }

    (k, k1, k2, err)
}
