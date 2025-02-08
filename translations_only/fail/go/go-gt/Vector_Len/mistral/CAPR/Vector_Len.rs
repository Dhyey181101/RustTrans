

use std::boxed::Box;
use std::mem::size_of;

struct Vector(Box<[i64; 4]>);

fn len(vector: &Vector) -> usize {
 let slice = vector.0.as_ref();
 (size_of::<[i64; 4]>() / size_of::<i64>()) * slice.len()
}

