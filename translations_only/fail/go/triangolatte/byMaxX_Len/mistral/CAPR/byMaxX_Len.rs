

use std::boxed::Box;

struct Point {
    x: f64,
    y: f64,
}

type Polygons = Vec<Vec<Point>>;

struct ByMaxX {
    data: Box<Polygons>,
}

fn size(by_max_x: &ByMaxX) -> usize {
    (**by_max_x.data).len()
}

fn main() {}

