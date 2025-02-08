
use std::f64;

struct Point {
    x: f64,
    y: f64,
}

fn polygon_area(data: &[Point]) -> f64 {
    let mut area = 0.0;
    let mut j = data.len() - 1;
    for i in 0..data.len() {
        area += data[i].x * data[j].y - data[i].y * data[j].x;
        j = i;
    }
    (area / 2.0).abs()
}

fn main() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 0.0, y: 1.0 },
    ];
    println!("Area: {}", polygon_area(&points));
}
