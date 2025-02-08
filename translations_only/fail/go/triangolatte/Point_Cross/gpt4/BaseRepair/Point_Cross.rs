
struct Point {
    X: f64,
    Y: f64,
}

fn cross(p: Box<Point>, r: Box<Point>) -> f64 {
    p.X * r.Y - p.Y * r.X
}
