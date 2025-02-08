

use std::boxed::Box;

struct Point {
    x: f64,
    y: f64,
}

type Polygons = Vec<Vec<Point>>;

fn polygons_less(polygons: &Polygons, i: usize, j: usize) -> bool {
    let mut x_max = 0.0;

    for k in &polygons[i] {
        if k.x > x_max {
            x_max = k.x;
        }
    }

    for k in &polygons[j] {
        if k.x > x_max {
            return false;
        }
    }

    true
}

