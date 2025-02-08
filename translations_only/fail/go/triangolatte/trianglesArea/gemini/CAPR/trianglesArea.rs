
fn triangles_area(t: &[f64]) -> f64 {
    let mut triangles_area = 0.0;
    for i in (0..t.len()).step_by(6) {
        triangles_area += (t[i] * (t[i + 3] - t[i + 5]) + t[i + 2] * (t[i + 5] - t[i + 1]) + t[i + 4] * (t[i + 1] - t[i + 3])) / 2.0;
    }
    triangles_area.abs()
}
