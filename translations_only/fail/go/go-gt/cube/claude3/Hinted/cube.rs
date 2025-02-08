
fn cube(x: f64) -> Option<f64> {
    if x.is_normal() {
        Some(x * x * x)
    } else {
        None
    }
}
