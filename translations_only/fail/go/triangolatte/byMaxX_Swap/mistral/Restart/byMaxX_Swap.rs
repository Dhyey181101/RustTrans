
use std::fmt;

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
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

fn main() {
    let mut polygons = vec![
        vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }],
        vec![Point { x: 3.0, y: 3.0 }, Point { x: 4.0, y: 4.0 }],
    ];

    let mut by_max_x = ByMaxX(polygons);
    by_max_x.swap(0, 1);

    println!("{:?}", by_max_x.0);
}
