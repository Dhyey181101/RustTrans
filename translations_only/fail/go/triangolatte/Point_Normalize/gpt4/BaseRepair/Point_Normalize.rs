
struct Point {
    x: f64,
    y: f64,
}

fn normalize(p: Box<Point>) -> Box<Point> {
    scale(p, 1.0)
}

fn scale(p: Box<Point>, f: f64) -> Box<Point> {
    let norm = f64::sqrt(p.x * p.x + p.y * p.y);
    Box::new(Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    })
}
