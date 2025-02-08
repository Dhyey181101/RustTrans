
pub fn swap<T>(slice: &mut [T], i: usize, j: usize) {
    slice.swap(i, j);
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub type ByMaxX = Vec<Point>;
