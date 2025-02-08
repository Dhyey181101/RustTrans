
struct Point {
    x: f64,
    y: f64,
}

struct ByMaxX(Vec<Vec<Point>>);

fn swap(polygons: &mut ByMaxX, i: usize, j: usize) {
    polygons.0.swap(i, j);
}
