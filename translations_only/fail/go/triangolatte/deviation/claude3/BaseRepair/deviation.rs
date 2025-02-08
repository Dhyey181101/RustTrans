
use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn deviation(data: Vec<Point>, holes: Vec<Vec<Point>>, t: Vec<f64>) -> (f64, f64, f64) {
    let calculated = triangles_area(&t);
    let mut actual = polygon_area(&data);
    for h in holes {
        actual -= polygon_area(&h);
    }

    let deviation = (calculated - actual).abs();
    (actual, calculated, deviation)
}

fn triangles_area(t: &[f64]) -> f64 {
    let mut triangles_area = 0.0;
    for i in (0..t.len()).step_by(6) {
        triangles_area += (t[i] * (t[i + 3] - t[i + 5])
            + t[i + 2] * (t[i + 5] - t[i + 1])
            + t[i + 4] * (t[i + 1] - t[i + 3]))
            .abs()
            / 2.0;
    }
    triangles_area
}

fn polygon_area(data: &[Point]) -> f64 {
    let mut area = 0.0;
    for (i, &Point { x, y }) in data.iter().enumerate() {
        let j = if i == data.len() - 1 { 0 } else { i + 1 };
        let Point { x: x_j, y: y_j } = data[j];
        area += x * y_j - y * x_j;
    }
    area.abs() / 2.0
}
