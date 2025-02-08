
fn cross(p: Point, r: Point) -> Option<f64> {
    Some(p.x * r.y - p.y * r.x)
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
