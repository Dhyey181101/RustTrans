
struct Point {
    x: f64,
    y: f64,
}

struct ByMaxX {
    polygons: Vec<Vec<Point>>,
}

impl ByMaxX {
    fn len(&self) -> usize {
        self.polygons.len()
    }
}

fn main() {}
