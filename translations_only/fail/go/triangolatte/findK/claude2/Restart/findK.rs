
use std::error::Error;

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn sub(p: Point, r: Point) -> Point {
    Point {
        x: p.x - r.x,
        y: p.y - r.y,
    }
}

fn cross(p: Point, r: Point) -> f64 {
    p.x * r.y - p.y * r.x
}

fn find_k(
    m: Point, 
    outer: Vec<Point>,
) -> (
    Point, 
    i32, 
    i32, 
    Result<(), Box<dyn Error>>
) {
    let mut k = Point { x: 0.0, y: 0.0 };
    let mut k1 = 0;
    let mut k2 = 0;

    for i in (0..outer.len()-1).rev() {
        let j = i + 1;

        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = sub(m, outer[i]);
        let v2 = sub(outer[j], outer[i]);

        let t1 = cross(v2, v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < k.x {
                k = Point {
                    x: t1 + m.x,
                    y: m.y
                };
                k1 = i as i32;
                k2 = j as i32;
                return (k, k1, k2, Ok(()));
            }
        } else {
            return (k, k1, k2, Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData, 
                "cannot calculate intersection, problematic data"
            ))));
        }
    }

    (k, k1, k2, Ok(()))
}
