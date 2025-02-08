
struct Point {
    x: f64,
    y: f64,
}

fn distance2(p: Point, r: Point) -> Option<f64> {
    let x_diff = p.x - r.x;
    let y_diff = p.y - r.y;
    let result = x_diff * x_diff + y_diff * y_diff;
    if result.is_finite() {
        Some(result)
    } else {
        None
    }
}
