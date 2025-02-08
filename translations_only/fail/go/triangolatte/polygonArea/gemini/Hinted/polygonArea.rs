
use std::f64::consts::PI;

fn polygon_area(data: &Vec<Point>) -> Option<f64> {
    let mut area = 0.0;
    for i in 0..data.len() {
        let j = (i + 1) % data.len();
        area += data[i].x * data[j].y - data[i].y * data[j].x;
    }
    Some(area.abs() / 2.0)
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
