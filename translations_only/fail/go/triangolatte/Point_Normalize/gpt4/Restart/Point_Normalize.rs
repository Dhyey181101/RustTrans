
struct Point {
    X: f64,
    Y: f64,
}

fn normalize(p: Box<Point>) -> Box<Point> {
    scale(p, 1.0)
}

fn scale(p: Box<Point>, f: f64) -> Box<Point> {
    let norm = f64::sqrt(p.X * p.X + p.Y * p.Y);
    Box::new(Point {
        X: p.X / norm * f,
        Y: p.Y / norm * f,
    })
}
