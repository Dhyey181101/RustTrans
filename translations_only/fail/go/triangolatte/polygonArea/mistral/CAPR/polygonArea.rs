
use std::f64;

type Point = (f64, f64);

fn polygon_area(data: Vec<Point>) -> f64 {
    let mut area = 0.0;
    let len = data.len();
    for i in 0..len {
        let j = if i == len - 1 { 0 } else { i + 1 };
        area += data[i].0 * data[j].1 - data[i].1 * data[j].0;
    }
    area.abs() / 2.0
}
