
use std::f64;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

fn polygon_area(data: Vec<Point>) -> f64 {
    let mut area = 0.0;
    let len = data.len();
    for i in 0..len {
        let j = if i == len - 1 { 0 } else { i + 1 };
        area += data[i].x * data[j].y - data[i].y * data[j].x;
    }
    area.abs() / 2.0
}

fn is_valid_polygon(data: &[Point]) -> bool {
    if data.len() < 3 {
        return false;
    }
    for i in 0..data.len() {
        let j = if i == data.len() - 1 { 0 } else { i + 1 };
        if data[i].x.is_nan() || data[i].y.is_nan() {
            return false;
        }
        if (data[i].x == 0.0 && data[i].y == 0.0)
            || (data[j].x == 0.0 && data[j].y == 0.0)
        {
            return false;
        }
    }
    true
}

fn main() {
    let points1 = vec![];
    let points2 = vec![Point::new(3.2258580014254e-310, -1.6832434884954795e+212)];
    let points3 = vec![Point::new(0.0, 9.456563898655606e-308)];
    let points4 = vec![
        Point::new(1.0, 1.0),
        Point::new(5.0, 1.0),
        Point::new(5.0, 5.0),
        Point::new(1.0, 5.0),
    ];

    println!("Area of polygon 1: {}", polygon_area(points1));
    println!("Area of polygon 2: {}", polygon_area(points2));
    println!("Area of polygon 3: {}", polygon_area(points3));
    println!("Area of polygon 4: {}", polygon_area(points4));
}

impl Mul for Point {
    type Output = f64;

    fn mul(self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point::new(self.x * other, self.y * other)
    }
}
