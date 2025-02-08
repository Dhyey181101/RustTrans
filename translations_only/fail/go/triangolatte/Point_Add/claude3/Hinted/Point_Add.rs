
struct Point {
    x: f64,
    y: f64,
}

fn add(p: Point, r: Point) -> Point {
    Point {
        x: p.x + r.x,
        y: p.y + r.y,
    }
}
