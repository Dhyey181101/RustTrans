
use std::f64;

fn create_boxed_float() -> Box<f64> {
    Box::new(3.1415)
}

fn main() {
    let b = create_boxed_float();
    println!("The value of boxed float is: {}", b);
}
