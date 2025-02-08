
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
        let x1 = t[i];
        let y1 = t[i+1];
        let x2 = t[i+2];
        let y2 = t[i+3];
        let x3 = t[i+4];
        let y3 = t[i+5];
        
        let area = (x1*(y2-y3) + x2*(y3-y1) + x3*(y1-y2)).abs() / 2.0;
        triangles_area += area;
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
    (area / 2.0).abs()  
}
