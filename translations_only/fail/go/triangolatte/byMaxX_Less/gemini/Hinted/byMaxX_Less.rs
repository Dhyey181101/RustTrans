
fn less(polygons: &Vec<Vec<Point>>, i: usize, j: usize) -> bool {
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

struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let polygons = vec![
        vec![Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }],
        vec![Point { x: 5.0, y: 6.0 }, Point { x: 7.0, y: 8.0 }],
    ];

    let result = less(&polygons, 0, 1);
    println!("{}", result);
}
