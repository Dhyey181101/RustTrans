
use std::f64::consts::PI;

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

fn main() {
    let p = Point { x: 0.0, y: 0.0 };
    let r = Point { x: PI, y: PI };
    
    let res = add(p, r);
    
    println!("{} {}", res.x, res.y);
}
