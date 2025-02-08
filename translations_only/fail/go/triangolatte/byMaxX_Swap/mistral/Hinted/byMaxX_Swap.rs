
use std::fmt;

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

type Polygons = Vec<Vec<Point>>;

struct ByMaxX(Polygons);

impl ByMaxX {
    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }
}

fn main() {}
