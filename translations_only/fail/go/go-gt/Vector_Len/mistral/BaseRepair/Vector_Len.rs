

use std::boxed::Box;
use std::mem::size_of;

struct Vector(Box<[i64]>);

fn vector_len(vector: &Vector) -> usize {
( *vector.0 ).len()
}

fn main() {
let v = Vector(Box::new([1, 2, 3]));
println!("Length of vector is: {}", vector_len(&v));
}

