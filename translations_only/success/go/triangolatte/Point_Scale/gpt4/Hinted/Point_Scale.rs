
struct Point {
    x: f64,
    y: f64,
}

fn scale(p: Box<Point>, f: f64) -> Box<Point> {
    let norm = f64::sqrt(p.x * p.x + p.y * p.y);
    if norm == 0.0 {
        panic!("Input is invalid, crash gracefully");
    }
    Box::new(Point {
        x: p.x / norm * f,
        y: p.y / norm * f,
    })
}
