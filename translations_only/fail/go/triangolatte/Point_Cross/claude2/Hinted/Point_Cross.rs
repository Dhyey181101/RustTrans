
struct Point {
    x: f64,
    y: f64,
}

fn cross(p: Point, r: Point) -> f64 {
    p.x * r.y - p.y * r.x
}
