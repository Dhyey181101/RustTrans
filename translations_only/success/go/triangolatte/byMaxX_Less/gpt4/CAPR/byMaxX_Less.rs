
struct Point {
    x: f64,
    y: f64,
}

struct ByMaxX(Vec<Vec<Point>>);

fn less(polygons: &ByMaxX, i: usize, j: usize) -> bool {
    let mut x_max = 0.0;

    for point in &polygons.0[i] {
        if point.x > x_max {
            x_max = point.x;
        }
    }

    for point in &polygons.0[j] {
        if point.x > x_max {
            return false;
        }
    }

    true
}
