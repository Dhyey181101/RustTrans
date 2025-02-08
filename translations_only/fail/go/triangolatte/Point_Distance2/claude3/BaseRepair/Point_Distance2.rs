
struct Point {
    x: f64,
    y: f64,
}

fn distance2(p: Point, r: Point) -> f64 {
    ((p.x - r.x) * (p.x - r.x) + (p.y - r.y) * (p.y - r.y))
}
