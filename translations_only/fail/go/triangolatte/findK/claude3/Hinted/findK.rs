
use std::error::Error;

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

fn find_k(
    m: Box<Point>,
    outer: Box<Vec<Point>>,
) -> (Box<Point>, usize, usize, Box<dyn Error>) {
    let mut k = Box::new(Point { x: 0.0, y: 0.0 });
    let mut k1 = 0;
    let mut k2 = 0;
    let mut err: Box<dyn Error> = Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "cannot calculate intersection, problematic data",
    ));

    for (i, j) in (0..outer.len()).zip(outer.len() - 1..outer.len()) {
        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = sub(&m, &outer[i]);
        let v2 = sub(&outer[j], &outer[i]);

        let t1 = cross(&v2, &v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < k.x {
                k = Box::new(Point {
                    x: t1 + m.x,
                    y: m.y,
                });
                k1 = i;
                k2 = j;
                err = Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "no error",
                ));
            }
        } else {
            err = Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "cannot calculate intersection, problematic data",
            ));
            return (k, k1, k2, err);
        }
    }

    (k, k1, k2, err)
}
