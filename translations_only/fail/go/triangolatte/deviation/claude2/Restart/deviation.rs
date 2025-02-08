
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

fn deviation(data: &[Point], holes: &[&[Point]], t: &[f64]) -> (f64, f64, f64) {
    let calculated = triangles_area(t);
    let mut actual = polygon_area(data);
    for h in holes {
        actual -= polygon_area(h);
    }
    
    let deviation = (calculated - actual).abs();
    (actual, calculated, deviation)
}

fn triangles_area(t: &[f64]) -> f64 {
    let mut triangles_area = 0.0;
    for i in (0..t.len()).step_by(6) {
        let tmp = t[i] * (t[i+3] - t[i+5]) + 
                  t[i+2] * (t[i+5] - t[i+1]) + 
                  t[i+4] * (t[i+1] - t[i+3]);
        triangles_area += tmp.abs() / 2.0;
    }
    triangles_area
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
