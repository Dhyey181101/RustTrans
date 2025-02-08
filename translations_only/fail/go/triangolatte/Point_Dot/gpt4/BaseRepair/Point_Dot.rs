
struct Point {
    X: f64,
    Y: f64,
}

fn dot(p: Box<Point>, r: Box<Point>) -> f64 {
    p.X * r.X + p.Y * r.Y
}
