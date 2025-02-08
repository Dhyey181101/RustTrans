
struct Point {
    X: f64,
    Y: f64,
}

fn sub(p: Box<Point>, r: Box<Point>) -> Box<Point> {
    Box::new(Point {
        X: p.X - r.X,
        Y: p.Y - r.Y,
    })
}
