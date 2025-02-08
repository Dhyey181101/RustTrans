
struct Point {
    x: f64,
    y: f64,
}

struct ByMaxX(Vec<Vec<Point>>);

fn len(polygons: &ByMaxX) -> usize {
    polygons.0.len()
}
