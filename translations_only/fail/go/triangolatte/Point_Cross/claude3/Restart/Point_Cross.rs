
struct Point {
    x: f64,
    y: f64,
}

fn cross(p: Box<Point>, r: Box<Point>) -> f64 {
    p.x * r.y - p.y * r.x
}
