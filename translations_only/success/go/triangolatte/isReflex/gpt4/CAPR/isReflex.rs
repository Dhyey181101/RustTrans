
struct Point {
    X: f64,
    Y: f64,
}

fn is_reflex(a: Box<Point>, b: Box<Point>, c: Box<Point>) -> bool {
    (b.X - a.X) * (c.Y - b.Y) - (c.X - b.X) * (b.Y - a.Y) < 0.0
}
