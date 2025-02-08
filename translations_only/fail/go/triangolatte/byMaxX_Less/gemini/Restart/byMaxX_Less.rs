
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn less_by_max_x(polygons: &Vec<Vec<Point>>, i: usize, j: usize) -> Ordering {
    let mut x_max = 0.0;

    for point in &polygons[i] {
        if point.x > x_max {
            x_max = point.x;
        }
    }

    for point in &polygons[j] {
        if point.x > x_max {
            return Ordering::Less;
        }
    }

    Ordering::Greater
}
