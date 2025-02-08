
use std::cmp::Ordering;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

type ByMaxX = Vec<Vec<Point>>;

fn swap(polygons: &mut ByMaxX, i: usize, j: usize) {
    if i < polygons.len() && j < polygons.len() {
        polygons.swap(i, j);
    }
}

fn max_x(points: &[Point]) -> f64 {
    points
        .iter()
        .map(|p| p.x)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap_or(0.0)
}

fn sort_by_max_x(mut polygons: ByMaxX) -> ByMaxX {
    polygons.sort_by(|a, b| {
        max_x(b).partial_cmp(&max_x(a)).unwrap_or(Ordering::Equal)
    });
    polygons
}
