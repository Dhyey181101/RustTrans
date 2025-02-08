
struct Point {
    x: f64,
    y: f64,
}

fn deviation(data: Vec<Point>, holes: Vec<Vec<Point>>, t: Vec<f64>) -> (f64, f64, f64) {
    let calculated = triangles_area(&t);
    let mut actual = polygon_area(&data);
    for h in holes {
        actual -= polygon_area(&h);
    }

    let deviation = (calculated - actual).abs();
    (actual, calculated, deviation)
}

fn triangles_area(t: &Vec<f64>) -> f64 {
    let mut triangles_area = 0.0;
    let mut i = 0;
    while i < t.len() {
        triangles_area += ((t[i] * (t[i + 3] - t[i + 5]) + t[i + 2] * (t[i + 5] - t[i + 1]) + t[i + 4] * (t[i + 1] - t[i + 3])) / 2.0).abs();
        i += 6;
    }
    triangles_area
}

fn polygon_area(data: &Vec<Point>) -> f64 {
    let mut area = 0.0;
    let mut i = 0;
    let mut j = data.len() - 1;
    while i < data.len() {
        area += data[i].x * data[j].y - data[i].y * data[j].x;
        j = i;
        i += 1;
    }
    (area / 2.0).abs()
}
