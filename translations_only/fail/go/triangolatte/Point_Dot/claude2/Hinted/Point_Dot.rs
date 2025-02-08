
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

fn dot(p: &Point, r: &Point) -> f64 {
    p.x * r.x + p.y * r.y 
}

fn main() {
    let p = Point { x: PI, y: PI };
    let r = Point { x: 2.0 * PI, y: 2.0 * PI };
    
    let result = dot(&p, &r);
    println!("{}", result); 
}
