
fn triangles_area(t: Vec<f64>) -> f64 {
    let mut triangles_area = 0.0;
    let mut i = 0;
    while i < t.len() {
        triangles_area += ((t[i] * (t[i + 3] - t[i + 5])
            + t[i + 2] * (t[i + 5] - t[i + 1])
            + t[i + 4] * (t[i + 1] - t[i + 3]))
            / 2.0)
            .abs();
        i += 6;
    }
    triangles_area
}
