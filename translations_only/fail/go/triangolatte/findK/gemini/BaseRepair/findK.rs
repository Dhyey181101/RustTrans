
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn cross(self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

fn find_k(m: Point, outer: &[Point]) -> Result<(Point, usize, usize), Box<dyn Error>> {
    for (i, j) in (0..outer.len()).rev().zip(0..outer.len()) {
        if outer[i].y > m.y || outer[j].y < m.y {
            continue;
        }

        let v1 = m.sub(outer[i]);
        let v2 = outer[j].sub(outer[i]);

        let t1 = v2.cross(v1) / v2.y;
        let t2 = v1.y / v2.y;

        if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
            if t1 - m.x < outer[i].x {
                return Ok((
                    Point {
                        x: t1 + m.x,
                        y: m.y,
                    },
                    i,
                    j,
                ));
            }
        } else {
            return Err(Box::new(IntersectionError));
        }
    }

    Err(Box::new(IntersectionError))
}

#[derive(Debug)]
struct IntersectionError;

impl fmt::Display for IntersectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot calculate intersection, problematic data")
    }
}

impl Error for IntersectionError {}
