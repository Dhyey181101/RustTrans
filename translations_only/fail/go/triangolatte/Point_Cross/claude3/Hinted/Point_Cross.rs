
struct Point {
    x: f64,
    y: f64,
}

fn cross(p: Point, r: Point) -> Option<f64> {
    let result = p.x * r.y - p.y * r.x;
    if result.is_finite() {
        Some(result)
    } else {
        None
    }
}
