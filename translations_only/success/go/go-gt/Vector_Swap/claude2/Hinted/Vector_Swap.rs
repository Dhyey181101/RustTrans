
use std::mem;

struct Vector(Vec<i64>);

fn swap(v: &mut Vector, i: usize, j: usize) {
    let tmp = v.0[i];
    v.0[i] = v.0[j];
    v.0[j] = tmp;
}
