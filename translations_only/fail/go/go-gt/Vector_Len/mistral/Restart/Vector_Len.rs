

use std::boxed::Box;
use std::ops::Range;

struct Vector(Box<[i64]>);

fn len(v: &Vector) -> i64 {
i64::from(v.0.len() as u32)
}

fn main() {
let v = Vector(Box::new([1, 2, 3]));
println!("Length of v: {}", len(&v));
}

