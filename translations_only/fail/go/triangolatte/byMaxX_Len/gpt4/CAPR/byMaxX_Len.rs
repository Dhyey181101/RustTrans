
struct Point {
    x: f64,
    y: f64,
}

struct ByMaxX {
    polygons: Vec<Vec<Box<Point>>>,
}

impl ByMaxX {
    fn len(&self) -> usize {
        self.polygons.len()
    }
}
