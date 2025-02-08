
struct Point {
    X: f64,
    Y: f64,
}

fn is_inside_triangle(a: Box<Point>, b: Box<Point>, c: Box<Point>, p: Box<Point>) -> bool {
    (c.X - p.X) * (a.Y - p.Y) - (a.X - p.X) * (c.Y - p.Y) >= 0.0 &&
    (a.X - p.X) * (b.Y - p.Y) - (b.X - p.X) * (a.Y - p.Y) >= 0.0 &&
    (b.X - p.X) * (c.Y - p.Y) - (c.X - p.X) * (b.Y - p.Y) >= 0.0
}
