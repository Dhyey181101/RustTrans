
fn distance2(p: Point, r: Point) -> Option<f64> {
    let dx = p.x - r.x;
    let dy = p.y - r.y;
    Some(dx * dx + dy * dy)
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
