

use std::vec::Vec;

struct Point {
    x: f64,
    y: f64,
}

type Polygons = Vec<Vec<Point>>;

struct ByMaxX(Polygons);

fn size(by_max_x: &ByMaxX) -> usize {
    by_max_x.0.len()
}

