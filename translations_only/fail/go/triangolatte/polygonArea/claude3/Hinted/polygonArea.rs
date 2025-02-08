
use std::f64;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn polygon_area(data: &[Point]) -> f64 {
    let mut area = 0.0;
    let len = data.len();
    for (i, point) in data.iter().enumerate() {
        let j = if i == len - 1 { 0 } else { i + 1 };
        let next_point = data.get(j).unwrap_or(&Point { x: 0.0, y: 0.0 });
        area += point.x * next_point.y - point.y * next_point.x;
    }
    area.abs() / 2.0
}
