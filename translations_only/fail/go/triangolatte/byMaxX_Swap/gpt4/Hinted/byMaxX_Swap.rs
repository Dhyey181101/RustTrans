
struct Point {
    x: f64,
    y: f64,
}

fn swap(polygons: &mut Vec<Box<[Point]>>, i: usize, j: usize) {
    if i >= polygons.len() || j >= polygons.len() {
        panic!("Input is invalid, crash gracefully");
    }
    polygons.swap(i, j);
}
