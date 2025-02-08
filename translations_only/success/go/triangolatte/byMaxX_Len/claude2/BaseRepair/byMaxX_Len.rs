
struct Point {
    x: f64,
    y: f64,
}

type ByMaxX = Vec<Vec<Point>>;

fn len(polygons: &ByMaxX) -> usize {
    polygons.len()
}
