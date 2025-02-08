
use std::f64;

fn triangles_area(t: &[f64]) -> f64 {
    let mut triangles_area = 0.0;
    for i in (0..t.len()).step_by(6) {
        let x1 = t[i];
        let y1 = t[i+1];
        let x2 = t[i+2]; 
        let y2 = t[i+3];
        let x3 = t[i+4];
        let y3 = t[i+5];
        
        let abs_area = f64::abs((x1*(y2-y3) + x2*(y3-y1) + x3*(y1-y2)) / 2.0);
        triangles_area += abs_area;
    }
    triangles_area 
}
