
fn dot(p: Point, r: Point) -> Option<f64> {
    Some(p.x * r.x + p.y * r.y)
}

struct Point {
    x: f64,
    y: f64,
}
