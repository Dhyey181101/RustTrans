
use std::boxed::Box;

type Vector = Vec<usize>;
type Matrix = Box<MatrixImpl>;

struct MatrixImpl {
    n: i64,
    a: Vec<i64>,
}

fn get(m: &MatrixImpl, i: usize, j: usize) -> i64 {
    m.a[i as usize * (m.n as usize) + j as usize]
}

fn n(m: &MatrixImpl) -> i64 {
    m.n
}
