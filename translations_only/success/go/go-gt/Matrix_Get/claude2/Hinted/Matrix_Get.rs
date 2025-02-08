
use std::boxed::Box;

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m.a[i * (m.n as usize) + j]
}

