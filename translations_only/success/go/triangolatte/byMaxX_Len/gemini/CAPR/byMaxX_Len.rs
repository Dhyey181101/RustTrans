
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.partial_cmp(&other.x).unwrap_or(Ordering::Equal)
            .then_with(|| self.y.partial_cmp(&other.y).unwrap_or(Ordering::Equal))
    }
}

pub fn len(polygons: &Vec<Vec<Point>>) -> usize {
    polygons.len()
}
