
use std::ops::IndexMut;

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: usize, a: Vec<i64>) -> Self {
        Self { n, a }
    }
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m.a[i * m.n + j]
}
