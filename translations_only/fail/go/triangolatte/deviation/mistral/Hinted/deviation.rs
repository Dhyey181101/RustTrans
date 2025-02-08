

use std::f64;

const EPSILON: f64 = 1e-10;

fn deviation(data: &[Point2<f64>], holes: &[Vec<Point2<f64>>], t: &[f64]) -> (f64, f64, f64) {
    let calculated = triangles_area(t);
    let mut actual = polygon_area(data);
    for h in holes {
        actual -= polygon_area(h);
    }

    let deviation = (actual - calculated).abs();
    (actual, calculated, deviation)
}

fn triangles_area(t: &[f64]) -> f64 {
    let mut triangles_area = 0.0;
    for i in (0..t.len()).step_by(6) {
        let a = Point2::new(t[i], t[i + 1]);
        let b = Point2::new(t[i + 2], t[i + 3]);
        let c = Point2::new(t[i + 4], t[i + 5]);

        triangles_area += area_of_triangle(&a, &b, &c);
    }
    triangles_area
}

fn polygon_area(data: &[Point2<f64>]) -> f64 {
    let mut area = 0.0;
    let len = data.len();
    for i in 0..len {
        let j = (i + 1) % len;
        area += data[i].x * data[j].y - data[i].y * data[j].x;
    }
    area.abs() / 2.0
}

fn area_of_triangle(a: &Point2<f64>, b: &Point2<f64>, c: &Point2<f64>) -> f64 {
    let ab = Point2::new(b.x - a.x, b.y - a.y);
    let ac = Point2::new(c.x - a.x, c.y - a.y);

    ab.x * ac.y - ab.y * ac.x
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point2<T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Clone> {
    x: T,
    y: T,
}

impl<T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Clone> Point2<T> {
    fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
}

impl Point2<f64> {
    fn abs_diff(&self, other: &Self) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }
}

