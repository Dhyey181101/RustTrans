

use std::boxed::Box;

struct Vector(Box<[i64]>);

fn new_vector(values: &[i64]) -> Vector {
let mut v = Vec::new();
v.extend_from_slice(values);
Vector(v.into_boxed_slice())
}

fn get(vector: &Vector, index: usize) -> i64 {
*vector.0.get(index).unwrap()
}

fn set(vector: &mut Vector, index: usize, value: i64) {
vector.0[index] = value;
}

fn swap(vector: &mut Vector, i: usize, j: usize) {
vector.0.swap(i, j);
}

fn extend_vector(vector: &mut Vector, values: &[i64]) {
let len = vector.0.len();
let mut new_vec = Vec::new();
new_vec.extend_from_slice(&vector.0[..len]);
new_vec.extend_from_slice(values);
*vector = Vector(new_vec.into_boxed_slice());
}

