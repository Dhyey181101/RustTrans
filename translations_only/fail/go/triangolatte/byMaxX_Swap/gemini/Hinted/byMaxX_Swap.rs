
pub fn swap<T>(arr: &mut [T], i: usize, j: usize) {
    arr.swap(i, j);
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub type ByMaxX = Vec<Point>;
