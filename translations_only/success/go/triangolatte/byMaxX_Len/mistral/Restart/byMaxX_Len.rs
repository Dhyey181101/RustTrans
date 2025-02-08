

use std::mem::size_of;

pub struct Polygons(Box<Vec<Vec<Point>>>);

pub struct Point {
x: f64,
y: f64,
}

pub fn polygons_len(polygons: &Polygons) -> usize {
polygons.0.len()
}

pub fn polygons_size(polygons: &Polygons) -> usize {
polygons.0.iter().map(|v| size_of::<Vec<Point>>() + v.iter().map(|p| size_of::<Point>()).sum::<usize>()).sum::<usize>()
}

