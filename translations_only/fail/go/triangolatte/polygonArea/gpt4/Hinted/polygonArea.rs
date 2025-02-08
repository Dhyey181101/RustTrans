
struct Point {
    X: f64,
    Y: f64,
}

fn polygon_area(data: Vec<Box<Point>>) -> f64 {
    let mut area = 0.0;
    let mut j = data.len() - 1;
    for i in 0..data.len() {
        area += data[i].X * data[j].Y - data[i].Y * data[j].X;
        j = i;
    }
    area.abs() / 2.0
}

fn main() {
}
