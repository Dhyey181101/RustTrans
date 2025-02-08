
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn deviation(data: &[Point], holes: &[&[Point]], t: &[f64]) -> (f64, f64, f64) {
    let calculated = triangles_area(t);
    let actual = polygon_area(data);
    let actual = holes.iter().fold(actual, |acc, hole| acc - polygon_area(hole));

    let deviation = calculated.abs() - actual.abs();
    (actual, calculated, deviation)
}

fn triangles_area(t: &[f64]) -> f64 {
    let mut triangles_area = 0.0;
    for i in (0..t.len()).step_by(6) {
        triangles_area += ((t[i] * (t[i + 3] - t[i + 5])
            + t[i + 2] * (t[i + 5] - t[i + 1])
            + t[i + 4] * (t[i + 1] - t[i + 3])) / 2.0).abs();
    }
    triangles_area
}

fn polygon_area(data: &[Point]) -> f64 {
    let mut area = 0.0;
    for i in 0..data.len() {
        let j = (i + 1) % data.len();
        area += data[i].x * data[j].y - data[i].y * data[j].x;
    }
    area.abs() / 2.0
}
