
use std::f64::consts::PI;

pub fn triangles_area(t: &[f64]) -> f64 {
    let mut area = 0.0;
    for i in (0..t.len()).step_by(6) {
        let x1 = t[i];
        let y1 = t[i + 1];
        let x2 = t[i + 2];
        let y2 = t[i + 3];
        let x3 = t[i + 4];
        let y3 = t[i + 5];
        let a = ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs() / 2.0;
        area += a;
    }
    area
}
