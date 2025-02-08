
use std::mem;

struct Point {
    x: f64,
    y: f64,
}

type ByMaxX = Vec<Vec<Point>>;

fn swap(polygons: &mut ByMaxX, i: usize, j: usize) {
    let tmp = mem::take(&mut polygons[i]);
    polygons[i] = mem::take(&mut polygons[j]);
    polygons[j] = tmp;
}
