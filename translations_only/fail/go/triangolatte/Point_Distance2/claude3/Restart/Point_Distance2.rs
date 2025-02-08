
struct Point {
    x: f64,
    y: f64,
}

fn distance2(p: Point, r: Point) -> f64 {
    let dx = p.x - r.x;
    let dy = p.y - r.y;
    dx * dx + dy * dy
}
