
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

fn deviation(data: &[Point], holes: &[&[Point]], t: &[f64]) -> (f64, f64, f64) {
    let calculated = triangles_area(t);
    let mut actual = polygon_area(data);
    for hole in holes {
        actual -= polygon_area(hole);
    }
    
    let deviation = (calculated - actual).abs();
    (actual, calculated, deviation)
}

fn triangles_area(t: &[f64]) -> f64 {
    let mut area = 0.0;
    for i in (0..t.len()).step_by(6) {
        area += (t[i] * (t[i+3] - t[i+5]) + 
                 t[i+2] * (t[i+5] - t[i+1]) +
                 t[i+4] * (t[i+1] - t[i+3])).abs() / 2.0;
    }
    area
}

fn polygon_area(points: &[Point]) -> f64 {
    let mut area = 0.0;
    let mut j = points.len() - 1;
    for i in 0..points.len() {
        area += points[i].x * points[j].y - points[i].y * points[j].x;
        j = i;
    }
    area.abs() / 2.0
}
