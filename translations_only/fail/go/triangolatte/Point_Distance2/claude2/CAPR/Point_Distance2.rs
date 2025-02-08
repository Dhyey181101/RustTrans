
struct Point {
    x: f64,
    y: f64,
}

fn distance_2(p: Point, r: Point) -> f64 {
    (p.x - r.x).powi(2) + (p.y - r.y).powi(2)
}
