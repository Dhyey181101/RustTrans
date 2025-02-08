
use std::f64::consts::PI;

fn polygon_area(data: &[Point]) -> f64 {
    let mut area = 0.0;
    for i in 0..data.len() {
        let j = (i + 1) % data.len();
        area += data[i].x * data[j].y - data[i].y * data[j].x;
    }
    area.abs() / 2.0
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
