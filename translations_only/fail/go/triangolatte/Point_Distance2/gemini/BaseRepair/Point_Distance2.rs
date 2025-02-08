
fn distance2(p: Point, r: Point) -> f64 {
    (p.x - r.x).powf(2.0) + (p.y - r.y).powf(2.0)
}

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}
