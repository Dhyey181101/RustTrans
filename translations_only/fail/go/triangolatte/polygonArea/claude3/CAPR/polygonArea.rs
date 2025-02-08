
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

fn polygon_area(data: Vec<Point>) -> f64 {
    let mut area = 0.0;
    let len = data.len();
    for i in 0..len {
        let j = if i == len - 1 { 0 } else { i + 1 };
        area += data[i].x * data[j].y - data[i].y * data[j].x;
    }
    area.abs() / 2.0
}
