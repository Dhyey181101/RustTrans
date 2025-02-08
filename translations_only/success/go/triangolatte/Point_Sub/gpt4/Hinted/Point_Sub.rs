
struct Point {
    x: f64,
    y: f64,
}

fn sub(p: Box<Point>, r: Box<Point>) -> Box<Point> {
    Box::new(Point {
        x: p.x - r.x,
        y: p.y - r.y,
    })
}

fn main() {}
