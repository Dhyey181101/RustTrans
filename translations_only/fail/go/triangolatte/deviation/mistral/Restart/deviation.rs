

use std::f64;
use std::ops::Sub;

type Point = (f64, f64);

fn deviation(data: Vec<Point>, holes: Vec<Vec<Point>>, t: Vec<f64>) -> (f64, f64, f64) {
    let calculated = triangles_area(t);
    let mut actual = polygon_area(data.clone());
    for h in &holes {
        actual = actual.sub(polygon_area(h.clone()));
    }

    let deviation = (calculated - actual).abs();
    (actual, calculated, deviation)
}

fn triangles_area(t: Vec<f64>) -> f64 {
    let mut triangles_area = 0.0;
    for i in (0..t.len()).step_by(6) {
        triangles_area += area_of_triangle(
            (t[i], t[i + 1]),
            (t[i + 2], t[i + 3]),
            (t[i + 4], t[i + 5]),
        );
    }
    triangles_area
}

fn polygon_area(data: Vec<Point>) -> f64 {
    let mut area = 0.0;
    let len = data.len();
    for i in 0..len {
        let j = (i + 1) % len;
        area += data[i].0 * data[j].1 - data[i].1 * data[j].0;
    }
    area.abs() / 2.0
}

fn area_of_triangle(a: Point, b: Point, c: Point) -> f64 {
    let ab = (b.0 - a.0, b.1 - a.1);
    let ac = (c.0 - a.0, c.1 - a.1);
    ab.0 * ac.1 - ab.1 * ac.0
}

