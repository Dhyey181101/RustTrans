
use std::boxed;

struct Point {
    x: f64,
    y: f64,
}

fn distance2(p: &Point, r: &Point) -> f64 {
    (p.x - r.x).powf(2.0) + (p.y - r.y).powf(2.0)
}

fn main() {
    let p = Box::new(Point { x: 1.0, y: 2.0 });
    let r = Box::new(Point { x: 3.0, y: 4.0 });
    println!("{}", distance2(&p, &r));
}
