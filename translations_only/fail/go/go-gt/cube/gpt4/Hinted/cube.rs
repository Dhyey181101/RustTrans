
fn cube(x: f64) -> Option<Box<f64>> {
    let result = x * x * x;
    if result.is_finite() {
        Some(Box::new(result))
    } else {
        None
    }
}

fn main() {
    println!("{:?}", cube(5.713228200591862e-260)); // Expected Output: Some(0.0)
    println!("{:?}", cube(1.3196968803886e-173)); // Expected Output: Some(0.0)
    println!("{:?}", cube(-1.917889161245789e+261)); // Expected Output: None
    println!("{:?}", cube(-3.363693660485351e+175)); // Expected Output: None
}
