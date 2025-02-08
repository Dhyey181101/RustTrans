
struct Point {
    X: f64,
    Y: f64,
}

fn add(p: Box<Point>, r: Box<Point>) -> Point {
    Point {
        X: p.X + r.X,
        Y: p.Y + r.Y,
    }
}
