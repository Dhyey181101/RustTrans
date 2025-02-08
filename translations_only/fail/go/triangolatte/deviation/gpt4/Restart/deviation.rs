
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

fn deviation(data: &[Point], holes: &[Vec<Point>], t: &[f64]) -> (f64, f64, f64) {
    let calculated = triangles_area(t);
    let mut actual = polygon_area(data);
    for h in holes {
        actual -= polygon_area(h);
    }

    let deviation = (calculated - actual).abs();
    (actual, calculated, deviation)
}

fn triangles_area(t: &[f64]) -> f64 {
    let mut area = 0.0;
    let mut i = 0;
    while i < t.len() {
        area += ((t[i] * (t[i + 3] - t[i + 5])
            + t[i + 2] * (t[i + 5] - t[i + 1])
            + t[i + 4] * (t[i + 1] - t[i + 3]))
            / 2.0)
            .abs();
        i += 6;
    }
    area
}

fn polygon_area(data: &[Point]) -> f64 {
    let mut area = 0.0;
    let mut j = data.len() - 1;
    for i in 0..data.len() {
        area += data[i].x * data[j].y - data[i].y * data[j].x;
        j = i;
    }
    area.abs() / 2.0
}

fn main() {
    // Example usage
    let data = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 0.0, y: 1.0 },
    ];
    let holes = vec![];
    let t = vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0];
    let (actual, calculated, deviation) = deviation(&data, &holes, &t);
    println!("Actual: {}, Calculated: {}, Deviation: {}", actual, calculated, deviation);
}
