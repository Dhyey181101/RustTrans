
fn dot(p: Box<Point>, r: Box<Point>) -> f64 {
    p.x * r.x + p.y * r.y
}

struct Point {
    x: f64,
    y: f64,
}
