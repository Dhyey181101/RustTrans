
use std::boxed::Box;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

type ByMaxX = Vec<Box<[Point]>>;

fn less(polygons: &ByMaxX, i: usize, j: usize) -> bool {
    let mut x_max = 0.0;

    for k in 0..polygons[i].len() {
        if polygons[i][k].x > x_max {
            x_max = polygons[i][k].x;
        }
    }

    for k in 0..polygons[j].len() {
        if polygons[j][k].x > x_max {
            return false;
        }
    }

    true
}
