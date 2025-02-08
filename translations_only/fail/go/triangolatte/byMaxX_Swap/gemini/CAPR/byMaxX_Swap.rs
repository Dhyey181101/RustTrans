
pub fn swap<T>(slice: &mut [T], i: usize, j: usize) {
    slice.swap(i, j);
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct ByMaxX(pub Vec<Point>);

impl ByMaxX {
    pub fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }
}
