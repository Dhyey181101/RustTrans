
use std::f64::*;

struct Point {
    x: f64,
    y: f64,
}

fn distance_2(p: Point, r: Point) -> Option<f64> {
    let dx = p.x - r.x;
    let dy = p.y - r.y;
    let result = dx * dx + dy * dy;

    if result.is_infinite() {
        None
    } else {
        Some(result) 
    }
}

