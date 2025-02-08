
struct Point {
    x: f64,
    y: f64,
}

fn polygon_area(data: Vec<Point>) -> f64 {
    let mut area = 0.0;
    let mut j = data.len() - 1;
    for i in 0..data.len() {
        area += data[i].x * data[j].y - data[i].y * data[j].x;
        j = i;
    }
    area.abs() / 2.0
}

fn main() {
}
