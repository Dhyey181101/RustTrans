
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

type ByMaxX = Vec<Vec<Point>>;

fn len(polygons: &ByMaxX) -> usize {
    polygons.len()
}

fn sort_by_max_x(polygons: &mut ByMaxX) {
    polygons.sort_by(|a, b| {
        let max_a = a.iter().max_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap_or(Ordering::Equal)).unwrap();
        let max_b = b.iter().max_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap_or(Ordering::Equal)).unwrap();
        max_a.x.partial_cmp(&max_b.x).unwrap_or(Ordering::Equal)
    });
}
