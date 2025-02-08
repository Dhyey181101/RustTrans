

use std::boxed::Box;

struct Point {
    x: f64,
    y: f64,
}

type Polygons = Vec<Vec<Point>>;

fn less(polygons: &mut Box<Polygons>) -> bool {
    let mut x_max = 0.0;

    for k in &polygons[0] {
        if k.x > x_max {
            x_max = k.x;
        }
    }

    for k in &polygons[1] {
        if k.x > x_max {
            return false;
        }
    }

    true
}
