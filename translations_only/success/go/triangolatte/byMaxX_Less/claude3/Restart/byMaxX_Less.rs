
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

type PolygonType = Vec<Vec<Point>>;

fn less(polygons: &PolygonType, i: usize, j: usize) -> bool {
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

fn sort_by_max_x(mut polygons: PolygonType) -> PolygonType {
    polygons.sort_by(|a, b| {
        let mut a_max = 0.0;
        let mut b_max = 0.0;

        for point in a.iter() {
            if point.x > a_max {
                a_max = point.x;
            }
        }

        for point in b.iter() {
            if point.x > b_max {
                b_max = point.x;
            }
        }

        a_max.partial_cmp(&b_max).unwrap_or(Ordering::Equal)
    });

    polygons
}
