
struct Point {
    x: f64,
    y: f64,
}

fn dot(p: Point, r: Point) -> Option<f64> {
    let result = p.x * r.x + p.y * r.y;
    if result.is_finite() {
        Some(result)
    } else {
        None
    }
}
