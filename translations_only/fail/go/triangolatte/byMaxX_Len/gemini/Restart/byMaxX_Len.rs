
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct ByMaxX(pub Vec<Point>);

impl ByMaxX {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
